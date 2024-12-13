use std::sync::Arc;
use std::time::Duration;
use chromiumoxide::Page;
use chromiumoxide::auth::Credentials;
use chromiumoxide::cdp::browser_protocol::fetch::{
    ContinueRequestParams, EventRequestPaused, FulfillRequestParams, HeaderEntry
};
use futures::StreamExt;

use base64::encode;

use crate::signups::setup::SignupContext;
use crate::phone_numbers::create_sms_verifier;
use crate::services::account_service::Account;
use crate::services::proxies_service;
use crate::utils::browser::{launch_browser, setup_browser_stealth, wait_for_element};
use crate::utils::retry::retry;

const URL: &str = "https://deviceandbrowserinfo.com/info_device";

pub async fn process_signup(
    id: &str,
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    let proxy = proxies_service::get_random_proxy(&context.proxies)?;
    let (host, port, username, password) = proxies_service::parse_proxy(&proxy)?;
    let p = format!("{}:{}", host, port);

    println!("{} | {}", proxy, p);
    let (browser, handler_task) = launch_browser(false, &p).await?;
    let sms_verifier = create_sms_verifier(
        &context.settings.integration.sms_verifier,
        &context.settings.integration.sms_verifier_api_key,
    )?;

    /*
    let (order_id, phone_number) = sms_verifier
        .get_phone_number("opt19", "DE")
        .await?;
    */
    println!("before create page");

    let formatted_url = format!("{}", URL);
    let page = Arc::new(browser.new_page("about:blank") // about:blank
        .await
        .expect("Failed to create page"));

    println!("after create page");

    let proxy_auth_value = format!(
        "Basic {}",
        encode(format!("{}:{}", username.clone().unwrap(), password.clone().unwrap()))
    );

    let mut request_paused = page.event_listener::<EventRequestPaused>().await.unwrap();
    let intercept_page = page.clone();
    let intercept_handle = tokio::spawn(async move {
        while let Some(event) = request_paused.next().await {
            println!("Intercepted Event: {:?}", event);
            /*
            let headers = vec![
                HeaderEntry::new("Proxy-Authorization".to_string(), proxy_auth_value.clone()),
            ];
        println!("Adding headers: {:?}", headers);
        */
        let params = ContinueRequestParams::builder()
            .request_id(event.request_id.clone())
//            .headers(headers)
            .build()
            .unwrap();
            if let Err(e) = intercept_page.execute(params).await {
                eprintln!("Failed to continue request: {}", e);
            }
        }
    });

    //setup_browser_stealth(&page).await?;

    if let (Some(username), Some(password)) = (username, password) {
        let credentials = Credentials { username, password };
        println!("before authentication");
        page.authenticate(credentials)
            .await
            .map_err(|e| e.to_string())?;
        println!("after authentication");
    }

    println!("before goto");
    page.goto(formatted_url)
        .await
        .map_err(|e| e.to_string())?;
    println!("after goto");
    println!("before navigation");
    page.wait_for_navigation().await.unwrap();
    println!("after navigation");

    /*
    fill_signup_form(&page, &account.firstname, &account.lastname, &account.email).await?;
    fill_phone_number_form(&page, &phone_number, "SE").await?;

    let sms_code = retry(
        || async { sms_verifier.get_sms_code(&order_id).await },
        10,
        Duration::from_secs(15)
    ).await?;

    fill_phone_number_code_form(&page, &sms_code).await?;
    */

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

async fn fill_signup_form(
    page: &Page,
    first_name: &str,
    last_name: &str,
    email: &str,
) -> Result<(), String> {
    wait_for_element(page, r#"input[data-test-first-name]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?
        .type_str(first_name)
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"input[data-test-last-name]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?
        .type_str(last_name)
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"input[data-test-email]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?
        .type_str(email)
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"div[data-test-age-confirm] div[data-test-check]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"button[data-test-next]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn fill_phone_number_form(
    page: &Page,
    phone_number: &str,
    country_code: &str,
) -> Result<(), String> {
    let number_without_country_code = &phone_number[3..];

    wait_for_element(page, r#"div[data-test-show-country-selector]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    let selector = format!(r#"div[data-test-calling-code="{}"]"#, country_code);
    wait_for_element(page, &selector, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"input[data-test-phone-number]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?
        .type_str(number_without_country_code)
        .await
        .map_err(|e| e.to_string())?;

    wait_for_element(page, r#"button[data-test-next]"#, Duration::from_secs(10))
        .await?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn fill_phone_number_code_form(
    page: &Page,
    sms_code: &str,
) -> Result<(), String> {
    let digits: Vec<char> = sms_code.chars().collect();

    for (index, digit) in digits.iter().enumerate() {
        let selector = format!(r#"input[data-test-code-digit{}]"#, index + 1);
        wait_for_element(page, &selector, Duration::from_secs(10))
            .await?
            .click()
            .await
            .map_err(|e| e.to_string())?
            .type_str(&digit.to_string())
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
