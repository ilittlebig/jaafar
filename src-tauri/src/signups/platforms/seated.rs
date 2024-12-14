use std::sync::Arc;

use crate::services::account_service::Account;
use crate::services::proxies_service;
use crate::signups::setup::SignupContext;
use crate::utils::browser::{launch_browser, create_page};

const URL: &str = "https://deviceandbrowserinfo.com/info_device";

pub async fn process_signup(
    id: &str,
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    let proxy = proxies_service::get_random_proxy(&context.proxies)?;
    let (host, port, username, password) = proxies_service::parse_proxy(&proxy)?;
    let proxy_address = format!("{}:{}", host, port);

    let (browser, handler_task) = launch_browser(false, &proxy_address).await?;
    let (page, intercept_task) = create_page(Arc::clone(&browser), username, password).await?;

    page.goto(URL)
        .await
        .map_err(|e| e.to_string())?;
    page.wait_for_navigation()
        .await
        .map_err(|e| e.to_string())?;

    /*
    browser.close()
        .await
        .map_err(|e| e.to_string())?;
    */

    handler_task
        .await
        .map_err(|e| format!("Handler task failed: {}", e))?;
    intercept_task
        .await
        .map_err(|e| format!("Intercept task failed: {}", e))?;

    Ok(())
}
