use reqwest::{Client, Method};
use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;

use super::SmsVerifier;
use crate::services::http_service;

pub struct SmsActivate<'a> {
    pub api_key: &'a str,
}

const GET_NUMBER_URL: &str = "https://api.sms-activate.ae/stubs/handler_api.php";
const GET_ACTIVATION_STATUS_URL: &str = "https://api.sms-activate.ae/stubs/handler_api.php";

#[async_trait]
impl SmsVerifier for SmsActivate<'_> {
    async fn get_phone_number(&self) -> Result<(String, String), String> {
        let params = build_params(&self.api_key, "getNumber", Some(HashMap::from([
            ("service", "ot"),
            ("country", "46"),
        ])));

        let response = http_service::send_request::<()>(
            GET_NUMBER_URL,
            Method::GET,
            None,
            Some(&params),
            None,
            None,
        ).await?;

        let response_text = http_service::response_to_text(response).await?;
        Ok(parse_phone_number_response(&response_text)?)
    }

    async fn get_sms_code(&self, id: &str) -> Result<String, String> {
        let params = build_params(&self.api_key, "getStatusV2", Some(HashMap::from([
            ("id", id),
        ])));

        let response = http_service::send_request::<()>(
            GET_ACTIVATION_STATUS_URL,
            Method::GET,
            None,
            Some(&params),
            None,
            None,
        ).await?;

        let response_json = http_service::response_to_json(response).await?;
        Ok(parse_sms_response(&response_json)?)
    }
}

fn build_params(api_key: &str, action: &str, additional: Option<HashMap<&str, &str>>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("api_key".to_string(), api_key.to_string());
    params.insert("action".to_string(), action.to_string());

    if let Some(additional_params) = additional {
        for (key, value) in additional_params {
            params.insert(key.to_string(), value.to_string());
        }
    }
    params
}

fn parse_phone_number_response(response: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = response.split(":").collect();
    if parts.len() == 3 && parts[0] == "ACCESS_NUMBER" {
        let activation_id = parts[1].to_string();
        let phone_number = parts[2].to_string();
        Ok((activation_id, phone_number))
    } else {
        Err("Invalid response format".to_string())
    }
}

fn parse_sms_response(response_json: &Value) -> Result<String, String> {
    if let Some(sms) = response_json.get("sms") {
        match sms {
            Value::Null => Err("SMS code is not available yet".to_string()),
            Value::String(sms_str) => Ok(sms_str.clone()),
            _ => Err("Unexpected 'sms' field type".to_string()),
        }
    } else {
        Err("Missing 'sms' field in response".to_string())
    }
}
