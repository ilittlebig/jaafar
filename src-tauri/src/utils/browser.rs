use std::time::Duration;
use tokio::time::timeout;
use futures::StreamExt;
use chromiumoxide::{Page, Element};
use chromiumoxide::browser::{Browser, BrowserConfig};

/// Launches a new browser instance and spawns a handler task to manage WebSocket communication.
///
/// # Returns
/// A `Result` containing:
/// - `(Browser, tokio::task::JoinHandle<()>)`: A tuple with the browser instance and the task handle for the WebSocket handler if successful.
/// - `String`: An error message if the browser launch fails.
pub async fn launch_browser(headless: bool) -> Result<(Browser, tokio::task::JoinHandle<()>), String> {
    let browser_config = if headless {
        BrowserConfig::builder()
            .arg("--disable-blink-features=AutomationControlled")
            .build()
            .map_err(|e| e.to_string())?
    } else {
        BrowserConfig::builder()
            .with_head()
            .arg("--disable-blink-features=AutomationControlled")
            .build()
            .map_err(|e| e.to_string())?
    };

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
