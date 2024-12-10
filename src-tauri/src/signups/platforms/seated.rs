use std::sync::Arc;

use crate::signups::setup::SignupContext;
use crate::services::account_service::Account;
use crate::utils::browser::{launch_browser, setup_browser_stealth};

const URL: &str = "https://go.seated.com/event-reminders";

pub async fn process_signup(
    id: &str,
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    let (mut browser, handler_task) = launch_browser(false).await?;

    let formatted_url = format!("{}/{}", URL, id);
    let page = browser.new_page(formatted_url)
        .await
        .map_err(|e| e.to_string())?;

    setup_browser_stealth(&page);

    browser.close()
        .await
        .map_err(|e| e.to_string())?;
    handler_task
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
