use std::time::Duration;
use tokio::time::{timeout, sleep};
use futures::StreamExt;
use chromiumoxide::{Page, Element};
use chromiumoxide::browser::{Browser, BrowserConfig};

use crate::services::proxies_service;
use crate::utils::profiles::get_random_profile;

/// Launches a browser instance and spawns a handler task for WebSocket communication.
///
/// # Arguments
/// - `headless`: Runs the browser in headless mode if `true`, otherwise with UI.
/// - `proxy`: The proxy string, which can include the following formats:
///   - `host:port` (unauthenticated proxy)
///   - `username:password@host:port` (authenticated proxy)
///   - Optional prefixes such as `http://` or `https://` may also be included.
///
/// # Returns
/// A `Result` with:
/// - `(Browser, tokio::task::JoinHandle<()>)`: On success.
/// - `String`: Error message on failure.
pub async fn launch_browser(
    headless: bool,
    proxy: &str,
) -> Result<(Browser, tokio::task::JoinHandle<()>), String> {
    let mut browser_config_builder = if headless {
        BrowserConfig::builder().new_headless_mode()
    } else {
        BrowserConfig::builder().with_head()
    };

    browser_config_builder = browser_config_builder
        .arg(format!("--proxy-server=http://{}", proxy))
        .arg("--disable-blink-features=AutomationControlled")
        .arg("--disable-software-rasterizer")
        .arg("--disable-dev-shm-usage")
        .arg("--disable-extensions")
        .arg("--disable-default-apps")
        .no_sandbox();

    let browser_config = browser_config_builder
        .build()
        .map_err(|e| e.to_string())?;

    let (browser, mut handler) = Browser::launch(browser_config)
        .await
        .map_err(|e| e.to_string())?;

    let handler_task = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    Ok((browser, handler_task))
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
pub async fn setup_browser_stealth(page: &Page) -> Result<(), String> {
    let browser_profile = get_random_profile();

    let stealth_script = format!(r#"
        // Override navigator properties
        Object.defineProperty(navigator, 'platform', {{ get: () => '{platform}' }});
        Object.defineProperty(navigator, 'vendor', {{ get: () => '{vendor}' }});

        // Override WebDriver
        setTimeout(() => {{
            Object.defineProperty(navigator, 'webdriver', {{
                get: () => undefined
            }});
        }}, 250);

        // Override WebGL properties
        const getParameter = WebGLRenderingContext.prototype.getParameter;
        WebGLRenderingContext.prototype.getParameter = function(parameter) {{
            if (parameter === 37445) return '{vendor}'; // VENDOR
            if (parameter === 37446) return '{webgl_renderer}'; // RENDERER
            return getParameter(parameter);
        }};
    "#,
        platform = browser_profile.platform,
        vendor = browser_profile.vendor,
        webgl_renderer = browser_profile.webgl_renderer
    );

    page.set_user_agent(browser_profile.ua)
        .await
        .map_err(|e| e.to_string())?;
    page.evaluate_on_new_document(stealth_script)
        .await
        .map_err(|e| e.to_string())?;

    // WebDriver is always present without this sleep for some reason
    sleep(Duration::from_millis(250)).await;

    Ok(())
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
