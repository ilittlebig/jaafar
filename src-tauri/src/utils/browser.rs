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

use crate::utils::profiles::get_random_profile;

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

    browser_config_builder = browser_config_builder
        .arg(format!("--proxy-server={}", proxy))
        .arg("--disable-blink-features=AutomationControlled")
        .arg("--disable-software-rasterizer")
        .arg("--disable-dev-shm-usage")
        .arg("--disable-extensions")
        .arg("--disable-default-apps")
        .enable_request_intercept()
        .disable_cache()
        .no_sandbox();

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
///
/// # Arguments
/// - `page`: A reference to the `Page` instance to apply stealth settings to.
///
/// # Returns
/// A `Result` containing:
/// - `()`: If the stealth configuration is successfully applied.
/// - `String`: An error message if the operation fails.
async fn setup_browser_stealth(page: &Page) -> Result<(), String> {
    let browser_profile = get_random_profile();

    let stealth_script = format!(r#"
        (function() {{
            // Override navigator properties
            Object.defineProperty(navigator, 'platform', {{ get: () => '{platform}' }});
            Object.defineProperty(navigator, 'vendor', {{ get: () => '{vendor}' }});
            Object.defineProperty(navigator, 'languages', {{ get: () => ['en-US', 'en'] }});
            Object.defineProperty(navigator, 'hardwareConcurrency', {{ get: () => {hardware_concurrency} }});

            // Remove webdriver property
            Object.defineProperty(navigator, 'webdriver', {{
                get: () => undefined,
                configurable: false,
                enumerable: false
            }});

            // Mock plugins and mimeTypes
            Object.defineProperty(navigator, 'plugins', {{
                get: () => [/* list of fake plugins */],
                configurable: false
            }});

            Object.defineProperty(navigator, 'mimeTypes', {{
                get: () => [/* list of fake MIME types */],
                configurable: false
            }});

            // Spoof WebGL properties
            const getParameter = WebGLRenderingContext.prototype.getParameter;
            WebGLRenderingContext.prototype.getParameter = function(parameter) {{
                if (parameter === 37445) return '{webgl_vendor}'; // VENDOR
                if (parameter === 37446) return '{webgl_renderer}'; // RENDERER
                return getParameter.call(this, parameter);
            }};

            // Override Function.prototype.toString
            const originalToString = Function.prototype.toString;
            Function.prototype.toString = function() {{
                if (this === Function.prototype.toString) {{
                    return 'function toString() {{ [native code] }}';
                }}
                return originalToString.apply(this, arguments);
            }};

            // Intercept Object.defineProperty to prevent redefining Error.prototype.stack
            const originalDefineProperty = Object.defineProperty;
            Object.defineProperty = function(target, prop, descriptor) {{
                if (target instanceof Error && prop === 'stack' && typeof descriptor.get === 'function') {{
                    // Replace the getter to prevent setting 'cdp = true'
                    return originalDefineProperty(target, prop, {{
                        get: function() {{
                            // Return the original stack without modifying 'cdp'
                            return Error.prototype.stack.call(this);
                        }},
                        configurable: true,
                        enumerable: false
                    }});
                }}
                return originalDefineProperty(target, prop, descriptor);
            }};
        }})();
    "#,
        platform = browser_profile.platform,
        vendor = browser_profile.vendor,
        webgl_vendor = browser_profile.vendor,
        webgl_renderer = browser_profile.webgl_renderer,
        hardware_concurrency = browser_profile.hardware_concurrency,
    );

    // Set a realistic user agent
    /*
    page.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .await
        .map_err(|e| e.to_string())?;
    */

    // Inject the stealth script before any other scripts run
    page.evaluate_on_new_document(stealth_script)
        .await
        .map_err(|e| e.to_string())?;

    // Allow some time for the script to take effect
    sleep(Duration::from_secs(2)).await;

    Ok(())
}

pub const HIDE_CHROME: &str = "";

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
