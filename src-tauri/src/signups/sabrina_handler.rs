use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, REFERER, ORIGIN};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use ua_generator::ua::spoof_ua;

use crate::services::files_service;
use crate::services::proxies_service;
use crate::services::http_service;
use crate::services::account_service::Account;
use crate::services::settings::Settings;
use crate::captchas::create_solver;
use crate::phone_numbers::create_sms_verifier;

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    variables: Value,
}

const CAPTCHA_WEBSITE_URL: &str = "https://embed.laylo.com";
const CAPTCHA_WEBSITE_KEY: &str = "6LfaRWApAAAAAPvWsG2tsIhBCLEdXyz_EUQtQily";
const CAPTCHA_TASK_TYPE: &str = "ReCaptchaV3TaskProxyLess";
const CAPTCHA_PAGE_ACTION: &str = "create_customer";

const REQUEST_URL: &str = "https://laylo.com/api/graphql";

const DROP_DATE: i64 = 1733386800000;
const FINGERPRINT_ID: &str = "3542ca6347494934b9e7be13fa3922e7";
const OPT_IN: bool = false;
const REFERRER: &str = "https://store.sabrinacarpenter.com/pages/tour";

const QUERY: &str = r#"
    mutation (
      $productId: ID!
      $userId: String
      $email: String
      $dropDate: Float
      $fingerprintId: String
      $phoneNumber: String
      $optIn: Boolean
      $referrer: String
      $captcha: String
      $instagramId: String
    ) {
      purchaseProduct(
        productId: $productId
        userId: $userId
        email: $email
        phoneNumber: $phoneNumber
        dropDate: $dropDate
        fingerprintId: $fingerprintId
        optIn: $optIn
        referrer: $referrer
        captcha: $captcha
        instagramId: $instagramId
      )
    }
"#;

pub async fn run(
    app_handle: tauri::AppHandle,
    product_id: &str,
    proxy_group: String,
    _mode: String,
) -> Result<(), String> {
    let accounts_path = files_service::resolve_path(&app_handle, "accounts.json")?;
    let proxies_path = files_service::resolve_path(&app_handle, &format!("proxies/{}.json", proxy_group))?;
    let settings_path = files_service::resolve_path(&app_handle, "settings.json")?;

    let accounts: Vec<Account> = files_service::read_json_file(accounts_path)?;
    let proxies: Vec<String> = files_service::read_json_file(proxies_path)?;
    let settings = Settings::new(settings_path)?;

    let integration = settings.integration;

    let sms_verifier = create_sms_verifier(&integration.sms_verifier, &integration.sms_verifier_api_key)?;
    let (activation_id, phone_number) = sms_verifier.get_phone_number().await?;
    println!("activation_id: {} | phone_number: {}", activation_id, phone_number);
    let sms_code = sms_verifier.get_sms_code(&activation_id).await?;
    println!("sms_code: {}", sms_code);

    /*
    for account in accounts {
        println!("Processing account: {}", account.email);

        if let Err(e) = process_account(
            &account,
            &proxies,
            &integration.captcha_solver,
            &integration.captcha_solver_api_key,
            integration.max_request_retries,
            &product_id
        ).await {
            println!("{}", e);
        }

        tokio::time::sleep(Duration::from_millis(integration.request_delay)).await;
    }
    */

    Ok(())
}

async fn process_account(
    account: &Account,
    proxies: &[String],
    captcha_solver: &str,
    captcha_solver_api_key: &str,
    max_request_retries: usize,
    product_id: &str,
) -> Result<(), String> {
    let mut attempts = 0;

    while attempts < max_request_retries {
        attempts += 1;
        println!("Attempt {}/{} for {}", attempts, max_request_retries, account.email);

        match try_process_account(
            account,
            proxies,
            product_id,
            captcha_solver,
            captcha_solver_api_key,
            max_request_retries,
        ).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if attempts == max_request_retries {
                    return Err(format!("Failed to process account {} after {} attempts: {}", account.email, attempts, e));
                }
                println!("Error on attempt {}: {}", attempts, e);
            }
        }

        // Wait 1 second before trying again
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn try_process_account(
    account: &Account,
    proxies: &[String],
    product_id: &str,
    captcha_solver: &str,
    captcha_solver_api_key: &str,
    max_request_retries: usize,
) -> Result<(), String> {
    let proxy_string = proxies_service::get_random_proxy(&proxies)?;

    let solver = create_solver(captcha_solver, captcha_solver_api_key)?;
    let captcha_solution = solver.solve(
        CAPTCHA_WEBSITE_URL,
        CAPTCHA_WEBSITE_KEY,
        CAPTCHA_TASK_TYPE,
        Some(CAPTCHA_PAGE_ACTION),
        Some(proxy_string),
    ).await?;

    let variables = serde_json::json!({
        "dropDate": DROP_DATE,
        "fingerprintId": FINGERPRINT_ID,
        "productId": product_id,
        "optIn": OPT_IN,
        "email": account.email,
        "referrer": REFERRER,
        "captcha": captcha_solution,
    });

    let graphql_request = GraphQLRequest {
        query: QUERY.to_string(),
        variables,
    };

    let proxy_string = proxies_service::get_random_proxy(&proxies)?;
    let proxy = proxies_service::transform_string_to_proxy(proxy_string)?;
    let user_agent = spoof_ua();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers.insert(REFERER, HeaderValue::from_static(CAPTCHA_WEBSITE_URL));
    headers.insert(ORIGIN, HeaderValue::from_static(CAPTCHA_WEBSITE_URL));

    let response = http_service::send_request_with_retries(
        REQUEST_URL,
        headers,
        proxy,
        &graphql_request,
        max_request_retries,
    ).await?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response for {}: {}", account.email, e))?;
    println!("Response for {}: {}", account.email, response_text);

    Ok(())
}
