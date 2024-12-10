use std::time::Duration;
use std::sync::Arc;
use chromiumoxide::Page;

use crate::signups::setup::SignupContext;
use crate::services::account_service::Account;
use crate::phone_numbers::create_sms_verifier;
use crate::utils::browser::{launch_browser, setup_browser_stealth, wait_for_element};
use crate::utils::retry::retry;

const URL: &str = "https://go.seated.com/event-reminders";

pub async fn process_signup(
    id: &str,
    account: Arc<Account>,
    context: Arc<SignupContext>
) -> Result<(), String> {
    let (mut browser, handler_task) = launch_browser(false, &context.proxies).await?;
    let sms_verifier = create_sms_verifier(
        &context.settings.integration.sms_verifier,
        &context.settings.integration.sms_verifier_api_key,
    )?;

    let (order_id, phone_number) = sms_verifier
        .get_phone_number("opt19", "DE")
        .await?;

    let formatted_url = format!("{}/{}", URL, id);
    let page = browser.new_page(formatted_url)
        .await
        .map_err(|e| e.to_string())?;

    setup_browser_stealth(&page).await?;

    fill_signup_form(&page, &account.firstname, &account.lastname, &account.email).await?;
    fill_phone_number_form(&page, &phone_number, "SE").await?;

    let sms_code = retry(
        || async { sms_verifier.get_sms_code(&order_id).await },
        10,
        Duration::from_secs(15)
    ).await?;

    fill_phone_number_code_form(&page, &sms_code).await?;

    /*
    browser.close()
        .await
        .map_err(|e| e.to_string())?;
    */
    handler_task
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
