use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, REFERER, ORIGIN};
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use std::thread::sleep;
use ua_generator::ua::spoof_ua;
use std::collections::HashMap;
use std::sync::Arc;

use crate::signups::setup::SignupContext;
use crate::services::proxies_service;
use crate::services::http_service;
use crate::services::account_service::Account;
use crate::phone_numbers::{SmsVerifier, create_sms_verifier};
use crate::utils::retry::retry;

pub async fn process_account(
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    let sms_verifier = create_sms_verifier(
        &context.settings.integration.sms_verifier,
        &context.settings.integration.sms_verifier_api_key,
    )?;

    let (order_id, phone_number) = sms_verifier
        .get_phone_number("opt19", "NL")
        .await?;

    println!("{} {}", order_id, phone_number);

    let body = serde_json::json!({
        "phoneNumber": phone_number,
    });

    /*
    let proxy_string = proxies_service::get_random_proxy(&context.proxies)?;
    let proxy = proxies_service::transform_string_to_proxy(proxy_string)?;
    */

    let url = "https://api.seated.com/oauth/verify";
    let apikey = "";

    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("apikey".to_string(), apikey.to_string());
    params.insert("premium_proxy".to_string(), "true".to_string());

    let response = http_service::send_request(
        "https://api.zenrows.com/v1/",
        Method::POST,
        None,
        Some(&params),
        Some(&body),
        None,
    ).await?;
    println!("{}", http_service::response_to_text(response).await?);

    let sms_code = retry(
        || async { sms_verifier.get_sms_code(&order_id).await },
        10,
        Duration::from_secs(15)
    ).await?;
    println!("SMS CODE: {}", sms_code);

    Ok(())
}
