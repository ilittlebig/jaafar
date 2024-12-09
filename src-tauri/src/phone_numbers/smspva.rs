use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use async_trait::async_trait;
use serde_json::Value;
use regex::Regex;

use super::SmsVerifier;
use crate::services::http_service;

pub struct SmsPva<'a> {
    pub api_key: &'a str,
}

const GET_NUMBER_URL: &str = "https://api.smspva.com/activation/number";
const GET_SMS_URL: &str = "https://api.smspva.com/activation/sms";

#[async_trait]
impl SmsVerifier for SmsPva<'_> {
    async fn get_phone_number(&self, service: &str, country: &str) -> Result<(String, String), String> {
        let headers = build_headers(&self.api_key);
        let formatted_url = format!("{}/{}/{}", GET_NUMBER_URL, country, service);

        let response = http_service::send_request::<()>(
            formatted_url.as_str(),
            Method::GET,
            Some(headers),
            None,
            None,
            None,
        ).await?;

        let response_json = http_service::response_to_json(response).await?;
        Ok(parse_phone_number_response(&response_json)?)
    }

    async fn get_sms_code(&self, id: &str) -> Result<String, String> {
        let headers = build_headers(&self.api_key);
        let formatted_url = format!("{}/{}", GET_SMS_URL, id);

        let response = http_service::send_request::<()>(
            formatted_url.as_str(),
            Method::GET,
            Some(headers),
            None,
            None,
            None,
        ).await?;

        let response_json = http_service::response_to_json(response).await?;
        Ok(parse_sms_response(&response_json)?)
    }
}

fn build_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_str(api_key).unwrap());
    headers
}

fn parse_phone_number_response(response_json: &Value) -> Result<(String, String), String> {
    let data = response_json
        .get("data")
        .ok_or("Missing 'data' field in response")?;

    let order_id = data
        .get("orderId")
        .and_then(|v| v.as_i64())
        .ok_or("Missing or invalid 'orderId' field")?
        .to_string();

    let phone_number = data
        .get("phoneNumber")
        .and_then(|v| v.as_str())
        .ok_or("Missing or invalid 'phoneNumber' field")?
        .to_string();

    let country_code = data
        .get("countryCode")
        .and_then(|v| v.as_str())
        .ok_or("Missing or invalid 'countryCode' field")?
        .to_string();

    let phone_number = format!("{}{}", country_code, phone_number);
    Ok((order_id, phone_number))
}

fn parse_sms_response(response_json: &Value) -> Result<String, String> {
    if let Some(description) = response_json
        .get("error")
        .and_then(|error| error.get("description"))
        .and_then(|desc| desc.as_str())
    {
        return Err(description.to_string());
    }

    let full_text = response_json
        .get("data")
        .and_then(|data| data.get("sms"))
        .and_then(|sms| sms.get("fullText"))
        .and_then(|value| value.as_str())
        .ok_or("Missing or invalid 'fullText' in response")?;

    let re = regex::Regex::new(r"\b\d{4,6}\b").map_err(|e| format!("Invalid regex: {}", e))?;
    if let Some(capture) = re.find(full_text) {
        return Ok(capture.as_str().to_string());
    }

    response_json
        .get("data")
        .and_then(|data| data.get("sms"))
        .and_then(|sms| sms.get("code"))
        .and_then(|value| value.as_str())
        .map(String::from)
        .ok_or("Could not find SMS code in response".to_string())
}
