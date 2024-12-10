use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::Page;

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
