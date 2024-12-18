use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::task::JoinHandle;

use futures::StreamExt;

use chromiumoxide::{Page, Element};
use chromiumoxide::auth::Credentials;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::fetch::{
    ContinueRequestParams, EventRequestPaused
};

use crate::stealth::build_stealth_script;
use crate::utils::profiles::get_random_profile;

static CHROME_ARGS: [&'static str; 58] = [
    "--no-sandbox",
    "--no-first-run",
    "--hide-scrollbars",
    // "--allow-pre-commit-input",
    // "--user-data-dir=~/.config/google-chrome",
    "--allow-running-insecure-content",
    "--autoplay-policy=user-gesture-required",
    "--ignore-certificate-errors",
    "--no-default-browser-check",
    "--no-zygote",
    "--disable-setuid-sandbox",
    "--disable-dev-shm-usage", // required or else docker containers may crash not enough memory
    "--disable-threaded-scrolling",
    "--disable-demo-mode",
    "--disable-dinosaur-easter-egg",
    "--disable-fetching-hints-at-navigation-start",
    "--disable-site-isolation-trials",
    "--disable-web-security",
    "--disable-threaded-animation",
    "--disable-sync",
    "--disable-print-preview",
    "--disable-partial-raster",
    "--disable-in-process-stack-traces",
    "--disable-v8-idle-tasks",
    "--disable-low-res-tiling",
    "--disable-speech-api",
    "--disable-smooth-scrolling",
    "--disable-default-apps",
    "--disable-prompt-on-repost",
    "--disable-domain-reliability",
    "--disable-component-update",
    "--disable-background-timer-throttling",
    "--disable-breakpad",
    "--disable-software-rasterizer",
    "--disable-extensions",
    "--disable-popup-blocking",
    "--disable-hang-monitor",
    "--disable-image-animation-resync",
    "--disable-client-side-phishing-detection",
    "--disable-component-extensions-with-background-pages",
    "--disable-ipc-flooding-protection",
    "--disable-background-networking",
    "--disable-renderer-backgrounding",
    "--disable-field-trial-config",
    "--disable-back-forward-cache",
    "--disable-backgrounding-occluded-windows",
    "--force-fieldtrials=*BackgroundTracing/default/",
    // "--enable-automation",
    "--log-level=3",
    "--enable-logging=stderr",
    "--enable-features=SharedArrayBuffer,NetworkService,NetworkServiceInProcess",
    "--metrics-recording-only",
    "--use-mock-keychain",
    "--force-color-profile=srgb",
    "--mute-audio",
    "--no-service-autorun",
    "--password-store=basic",
    "--export-tagged-pdf",
    "--no-pings",
    "--use-gl=swiftshader",
    "--disable-features=InterestFeedContentSuggestions,PrivacySandboxSettings4,AutofillServerCommunication,CalculateNativeWinOcclusion,OptimizationHints,AudioServiceOutOfProcess,IsolateOrigins,site-per-process,ImprovedCookieControls,LazyFrameLoading,GlobalMediaControls,DestroyProfileOnBrowserClose,MediaRouter,DialMediaRouteProvider,AcceptCHFrame,AutoExpandDetailsElement,CertificateTransparencyComponentUpdater,AvoidUnnecessaryBeforeUnloadCheckSync,Translate"
];

pub fn is_chromium_based(browser_name: &str) -> bool {
    matches!(browser_name, "Chrome" | "Google Chrome" | "Microsoft Edge")
}

/// Launches a browser instance and spawns a handler task for WebSocket communication.
///
/// # Arguments
/// - `headless`: Runs the browser in headless mode if `true`, otherwise with UI.
/// - `proxy`: The proxy string, which has to be in the following format:
///   - `host:port` (unauthenticated proxy)
///
/// # Returns
/// A `Result` with:
/// - `(Browser, tokio::task::JoinHandle<()>)`: On success.
/// - `String`: Error message on failure.
pub async fn launch_browser(
    headless: bool,
    proxy: &str,
) -> Result<(Arc<Browser>, JoinHandle<()>), String> {
    let mut browser_config_builder = if headless {
        BrowserConfig::builder()
    } else {
        BrowserConfig::builder().with_head()
    };

    let mut chrome_args = Vec::from(CHROME_ARGS.map(|e| e.replace("://", "=").to_string()));
    browser_config_builder = browser_config_builder
        .disable_default_args()
        .enable_request_intercept()
        .args(chrome_args)
        .arg(format!("--proxy-server={}", proxy));

    let browser_config = browser_config_builder
        .build()
        .expect("Failed to build browser config");

    let (browser, mut handler) = Browser::launch(browser_config)
        .await
        .expect("Failed to launch browser");

    let handler_task = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let browser = Arc::new(browser);
    Ok((browser, handler_task))
}

/// Creates a new browser page, sets up request interception, authenticates, and configures stealth mode.
///
/// # Arguments
/// - `browser` - A shared reference to the `Browser` instance.
/// - `username` - Optional username page authentication.
/// - `password` - Optional password page authentication.
///
/// # Returns
/// Returns a `Result` containing:
/// - `(Arc<Page>, JoinHandle<()>)` on success.
/// - `String` with an error message on failure.
pub async fn create_page(
    browser: Arc<Browser>,
    username: Option<String>,
    password: Option<String>,
) -> Result<(Arc<Page>, JoinHandle<()>), String> {
    let page = Arc::new(browser.new_page("about:blank")
        .await
        .expect("Failed to create page"));

    let mut request_paused = page.event_listener::<EventRequestPaused>()
        .await
        .expect("Failed to ...");

    let intercept_page = page.clone();
    let intercept_task = tokio::spawn(async move {
        while let Some(event) = request_paused.next().await {
            let params = ContinueRequestParams::builder()
                .request_id(event.request_id.clone())
                .build()
                .expect("Failed to ...");

            if let Err(e) = intercept_page.execute(params).await {
                eprintln!("Failed to continue request: {}", e);
            }
        }
    });

    setup_browser_stealth(&page).await?;
    authenticate_page(&page, username, password).await?;

    Ok((page, intercept_task))
}

/// Waits for an element matching the selector to appear on the page.
///
/// # Arguments
/// - `page`: The `Page` instance to search for the element.
/// - `selector`: The CSS selector to find the element.
/// - `timeout_duration`: How long to wait before timing out.
///
/// # Returns
/// The element handle if found, or an error if the timeout is reached.
pub async fn wait_for_element(
    page: &Page,
    selector: &str,
    timeout_duration: Duration,
) -> Result<Element, String> {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout_duration {
        if let Ok(element) = page.find_element(selector).await {
            return Ok(element);
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(format!(
        "Timeout: Element with selector '{}' not found within {:?}",
        selector, timeout_duration
    ))
}

/// Configures the browser page to operate in stealth mode by overriding browser properties
/// commonly checked by anti-bot systems.
async fn setup_browser_stealth(page: &Page) -> Result<(), String> {
    let browser_profile = get_random_profile();
    let stealth_script = build_stealth_script(&browser_profile);

    page.set_user_agent(browser_profile.ua)
        .await
        .map_err(|e| e.to_string())?;
    page.evaluate_on_new_document(stealth_script)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn authenticate_page(
    page: &Page,
    username: Option<String>,
    password: Option<String>
) -> Result<(), String> {
    if let (Some(username), Some(password)) = (username, password) {
        let credentials = Credentials { username, password };
        page.authenticate(credentials)
            .await
            .expect("Failed to authenticate the page");
    }
    Ok(())
}
