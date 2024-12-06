use reqwest::Client;
use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;

use super::SmsVerifier;

pub struct SmsActivate {
    pub api_key: String,
}

const GET_NUMBER_URL: &str = "https://api.sms-activate.ae/stubs/handler_api.php";
const GET_ACTIVATION_STATUS_URL: &str = "https://api.sms-activate.ae/stubs/handler_api.php";

#[async_trait]
impl SmsVerifier for SmsActivate {
    async fn get_phone_number(&self) -> Result<(String, String), String> {
        let client = Client::new();

        let a = "getNumber".to_string();
        let b = "ot".to_string();
        let c = "46".to_string();

        let mut params = HashMap::new();
        params.insert("api_key", &self.api_key);
        params.insert("action", &a);
        params.insert("service", &b);
        params.insert("country", &c);

        let response = client
            .get(GET_NUMBER_URL)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let (activation_id, phone_number) = parse_response(&response_text)?;
        Ok((activation_id, phone_number))
    }

    async fn get_sms_code(&self, activation_id: &str) -> Result<String, String> {
        let client = Client::new();

        let a = "getStatusV2".to_string();
        let b = activation_id.to_string();

        let mut params = HashMap::new();
        params.insert("api_key", &self.api_key);
        params.insert("action", &a);
        params.insert("id", &b);

        let response = client
            .get(GET_ACTIVATION_STATUS_URL)
            .query(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let json: Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response as JSON: {}", e))?;

        if let Some(sms) = json.get("sms") {
            if sms.is_null() {
                return Err("SMS code is not available yet".to_string());
            }
            if let Some(sms_str) = sms.as_str() {
                return Ok(sms_str.to_string());
            }
        }

        Err("Unexpected response format or missing 'sms' field".to_string())
    }
}

fn parse_response(response: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = response.split(":").collect();
    if parts.len() == 3 && parts[0] == "ACCESS_NUMBER" {
        let activation_id = parts[1].to_string();
        let phone_number = parts[2].to_string();
        Ok((activation_id, phone_number))
    } else {
        Err("Invalid response format".to_string())
    }
}
