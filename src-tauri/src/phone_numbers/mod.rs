pub mod smsactivate;

use async_trait::async_trait;
use smsactivate::SmsActivate;

#[async_trait]
pub trait SmsVerifier: Send + Sync {
    async fn get_phone_number(&self) -> Result<(String, String), String>;
    async fn get_sms_code(&self, id: &str) -> Result<String, String>;
}

pub fn create_sms_verifier<'a>(
    sms_verifier: &str,
    sms_verifier_api_key: &'a str
) -> Result<Box<dyn SmsVerifier + 'a>, String> {
    match sms_verifier {
        "sms-activate" => Ok(Box::new(SmsActivate {
            api_key: sms_verifier_api_key,
        })),
        _ => Err(format!("Unsupported SMS verifier: {}", sms_verifier))
    }
}
