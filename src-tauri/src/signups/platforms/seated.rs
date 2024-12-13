use std::sync::Arc;
use std::time::Duration;
use futures::StreamExt;

use chromiumoxide::Page;
use chromiumoxide::auth::Credentials;
use chromiumoxide::cdp::browser_protocol::fetch::{
    ContinueRequestParams, EventRequestPaused
};

use crate::services::account_service::Account;
use crate::services::proxies_service;
use crate::signups::setup::SignupContext;
use crate::utils::browser::{launch_browser, setup_browser_stealth};

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
    let page = Arc::new(browser.new_page("about:blank")
        .await
        .expect("Failed to create page"));

    let mut request_paused = page.event_listener::<EventRequestPaused>()
        .await
        .expect("Failed to ...");

    let intercept_page = page.clone();
    let intercept_handle = tokio::spawn(async move {
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

    if let (Some(username), Some(password)) = (username, password) {
        let credentials = Credentials { username, password };
        page.authenticate(credentials)
            .await
            .map_err(|e| e.to_string())?;
    }

    setup_browser_stealth(&page).await?;

    let formatted_url = format!("{}", URL);
    page.goto(formatted_url)
        .await
        .map_err(|e| e.to_string())?;
    page.wait_for_navigation().await.unwrap();

    /*
    browser.close()
        .await
        .map_err(|e| e.to_string())?;
    */

    handler_task
        .await
        .map_err(|e| e.to_string())?;
    intercept_handle
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
