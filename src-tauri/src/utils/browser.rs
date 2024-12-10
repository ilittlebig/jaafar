use std::time::Duration;
use tokio::time::timeout;
use futures::StreamExt;
use chromiumoxide::{Page, Element};
use chromiumoxide::browser::{Browser, BrowserConfig};
use ua_generator::ua::spoof_ua;

use crate::services::proxies_service;

/// Launches a browser instance and spawns a handler task for WebSocket communication.
///
/// # Arguments
/// - `headless`: Runs the browser in headless mode if `true`, otherwise with UI.
/// - `proxies`: A reference to a list of proxy strings.
///
/// # Returns
/// A `Result` with:
/// - `(Browser, tokio::task::JoinHandle<()>)`: On success.
/// - `String`: Error message on failure.
pub async fn launch_browser(
    headless: bool,
    proxies: &Vec<String>
) -> Result<(Browser, tokio::task::JoinHandle<()>), String> {
    let proxy_string = proxies_service::get_random_proxy(proxies)?;
    let proxy = proxies_service::format_proxy(proxy_string, true)?;

    let mut browser_config_builder = if headless {
        BrowserConfig::builder()
    } else {
        BrowserConfig::builder().with_head()
    };

    let user_agent = spoof_ua();
    browser_config_builder = browser_config_builder
        .arg(format!("--proxy-server=http={}", proxy))
        .arg(format!("--user-agent={}", user_agent))
        .arg("--disable-blink-features=AutomationControlled")
        .arg("--disable-software-rasterizer")
        .arg("--no-sandbox")
        .arg("--disable-dev-shm-usage")
        .arg("--disable-extensions")
        .arg("--disable-default-apps");

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
    let stealth_script = r#"
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined
        });
    "#;

    page.evaluate(stealth_script)
        .await
        .map_err(|e| e.to_string())?;
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
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Err(format!(
        "Timeout: Element with selector '{}' not found within {:?}",
        selector, timeout_duration
    ))
}
