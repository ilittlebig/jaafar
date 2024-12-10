use std::sync::Arc;

use crate::signups::setup::SignupContext;
use crate::services::account_service::Account;
use crate::signups::platforms::seated::process_signup;

pub async fn process_account(
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    process_signup("1c16af50-df45-4d4f-8848-b626888c8186", account, context).await?;

    /*
    let sms_verifier = create_sms_verifier(
        &context.settings.integration.sms_verifier,
        &context.settings.integration.sms_verifier_api_key,
    )?;

    let (order_id, phone_number) = sms_verifier
        .get_phone_number("opt19", "NL")
        .await?;

    let body = serde_json::json!({
        "phoneNumber": phone_number,
    });

    /*
    let proxy_string = proxies_service::get_random_proxy(&context.proxies)?;
    let proxy = proxies_service::transform_string_to_proxy(proxy_string)?;
    */

    let url = "https://api.seated.com/oauth/verify";
    let apikey = "6b854f7387d9098896275ef57369b35c374ad5d4";

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
    */

    Ok(())
}
