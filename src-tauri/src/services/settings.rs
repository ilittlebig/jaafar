use serde::Deserialize;
use std::path::PathBuf;

use crate::services::files_service;

#[derive(Deserialize)]
pub struct IntegrationSettings {
    pub captcha_solver: String,
    pub captcha_solver_api_key: String,
    pub sms_verifier: String,
    pub sms_verifier_api_key: String,
    pub request_delay: u64,
    pub max_request_retries: usize,
    pub entry_limit: usize,
    pub imap_email: String,
    pub imap_password: String,
    pub webhook: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub integration: IntegrationSettings,
}

impl Settings {
    pub fn new(settings_path: PathBuf) -> Result<Self, String> {
        Ok(files_service::read_json_file(settings_path)?)
    }
}
