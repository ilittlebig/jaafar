use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;
use std::time::Duration;
use tokio::sync::Semaphore;

use crate::utils::retry::retry;
use crate::services::files_service;
use crate::services::proxies_service;
use crate::services::settings::Settings;
use crate::services::account_service::Account;

pub struct SignupContext {
    pub accounts: Vec<Account>,
    pub proxies: Vec<String>,
    pub settings: Settings,
}

const RETRIES: usize = 3;
const DURATION_BETWEEN_RETRIES: Duration = Duration::from_secs(1);

/// Sets up the signup context by loading accounts, proxies and settings.
///
/// # Arguments
/// - `app_handle`: Reference to the Tauri application handle.
/// - `proxy_group`: Name of the proxy group to load proxies from.
///
/// # Returns
/// A `Result` containing:
/// - `Arc<SignupContext>`: If the setup is successful.
/// - `String`: Error message if loading any of the files fails.
pub fn setup_signup_context(
    app_handle: &tauri::AppHandle,
    proxy_group: String
) -> Result<Arc<SignupContext>, String> {
    let accounts_path = files_service::resolve_path(&app_handle, "accounts.json")?;
    let proxies_path = files_service::resolve_path(&app_handle, &format!("proxies/{}.json", proxy_group))?;
    let settings_path = files_service::resolve_path(&app_handle, "settings.json")?;

    let accounts: Vec<Account> = files_service::read_json_file(accounts_path)?;
    let proxies: Vec<String> = files_service::read_json_file(proxies_path)?;
    let settings = Settings::new(settings_path)?;

    Ok(Arc::new(SignupContext {
        accounts,
        proxies,
        settings,
    }))
}

/// Runs the signup process for all accounts using the given function.
///
/// # Arguments
/// - `context`: Shared `SignupContext`.
/// - `process_account`: Function that processes a single account.
/// - `semaphore_limit`: Maximum number of concurrent tasks.
///
/// # Returns
/// A `Result` with `()` on success or an error message on failure.
pub fn run_signup<F>(
    context: Arc<SignupContext>,
    process_account: F,
    semaphore_limit: usize,
) -> Result<(), String>
where
    F: Fn(Arc<Account>, Arc<SignupContext>) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync
        + 'static
        + Clone,
{
    let semaphore = Arc::new(Semaphore::new(semaphore_limit));
    let accounts = context.accounts.clone();

    for account in accounts.into_iter().map(Arc::new) {
        let semaphore = Arc::clone(&semaphore);
        let context = Arc::clone(&context);
        let process_account = process_account.clone();

        tokio::spawn(async move {
            let _permit = semaphore.acquire().await;
            match retry(
                || async {
                    let context = Arc::clone(&context);
                    let account = account.clone();
                    process_account(account, context).await
                },
                RETRIES,
                DURATION_BETWEEN_RETRIES,
            ).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error processing account {}: {}", account.email, e),
            }
        });
    }

    Ok(())
}
