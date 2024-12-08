use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, REFERER, ORIGIN};
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use ua_generator::ua::spoof_ua;
use std::collections::HashMap;
use std::sync::Arc;

use crate::signups::setup::SignupContext;
use crate::services::proxies_service;
use crate::services::http_service;
use crate::services::account_service::Account;
use crate::phone_numbers::{SmsVerifier, create_sms_verifier};
use crate::utils::retry::retry;

/*
let sms_verifier = match create_sms_verifier(
    &integration.sms_verifier,
    &integration.sms_verifier_api_key
) {
    Ok(result) => result,
    Err(e) => {
        println!("Failed to create the SMS verifier: {}", e);
        continue;
    },
};

let (activation_id, phone_number) = match retry(
    || async { sms_verifier.get_phone_number("are", "16").await },
    3,
    Duration::from_secs(3),
).await {
    Ok(result) => result,
    Err(e) => {
        println!("Failed to get a phone number: {}", e);
        continue;
    },
};
*/

pub async fn process_account(
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    /*
    let mut attempts = 0;

    while attempts < max_request_retries {
        attempts += 1;
        println!("Attempt {}/{} for {}", attempts, max_request_retries, account.email);

        match try_process_account(
            account,
            proxies,
            activation_id,
            phone_number,
            max_request_retries,
            sms_verifier,
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
    */

    Ok(())
}

async fn try_process_account(
    account: &Account,
    proxies: &[String],
    activation_id: &str,
    phone_number: &str,
    max_request_retries: usize,
    sms_verifier: &dyn SmsVerifier,
) -> Result<(), String> {
    let proxy_string = proxies_service::get_random_proxy(&proxies)?;

    let body = serde_json::json!({
        "phoneNumber": format!("+{}", phone_number),
    });

    let proxy_string = proxies_service::get_random_proxy(&proxies)?;
    let proxy = proxies_service::transform_string_to_proxy(proxy_string)?;
    let user_agent = spoof_ua();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers.insert(REFERER, HeaderValue::from_static("https://go.seated.com/"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://go.seated.com/"));

    let url = "https://api.seated.com/oauth/verify";
    let apikey = "6b854f7387d9098896275ef57369b35c374ad5d4";

    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("apikey".to_string(), apikey.to_string());

    let response = http_service::send_request(
        "https://api.zenrows.com/v1/",
        Method::POST,
        Some(headers),
        Some(&params),
        Some(&body),
        None,
    ).await?;
    println!("{}", http_service::response_to_text(response).await?);

    let sms_code = retry(
        || async { sms_verifier.get_sms_code(&activation_id).await },
        5,
        Duration::from_secs(5)
    ).await?;
    println!("{}", sms_code);

    /*
    let variables = serde_json::json!({
        "dropDate": DROP_DATE,
        "fingerprintId": FINGERPRINT_ID,
        "productId": product_id,
        "optIn": OPT_IN,
        "email": account.email,
        "referrer": REFERRER,
        "captcha": captcha_solution,
    });
    */

    let proxy_string = proxies_service::get_random_proxy(&proxies)?;
    let proxy = proxies_service::transform_string_to_proxy(proxy_string)?;

    /*
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers.insert(REFERER, HeaderValue::from_static(CAPTCHA_WEBSITE_URL));
    headers.insert(ORIGIN, HeaderValue::from_static(CAPTCHA_WEBSITE_URL));
    */

    /*
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
    */

    Ok(())
}
