pub mod active;
pub mod setup;
pub mod platforms;

use active::*;
use crate::signups::setup::{setup_signup_context, run_signup};

#[tauri::command]
pub async fn sabrina_hallenstadion(app_handle: tauri::AppHandle, proxy_group: String, _mode: String) -> Result<(), String> {
    let context = setup_signup_context(&app_handle, proxy_group)?;
    let entry_limit = context.settings.integration.entry_limit;

    run_signup(
        context,
        |account, context| {
            Box::pin(dead_and_company_handler::process_account(account, context))
        },
        entry_limit
    )
}
