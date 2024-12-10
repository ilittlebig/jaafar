use serde::{Serialize, Deserialize};
use reqwest::Method;
use async_trait::async_trait;

use super::CaptchaSolver;
use crate::services::http_service;

pub struct CapSolver {
    pub client_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapSolverTask<'a> {
    #[serde(rename = "type")]
    task_type: &'a str,
    #[serde(rename = "websiteURL")]
    website_url: &'a str,
    website_key: &'a str,
    page_action: Option<&'a str>,
    proxy: Option<&'a String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapSolverRequest<'a> {
    client_key: &'a str,
    task: CapSolverTask<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetTaskResultRequest<'a> {
    client_key: &'a str,
    task_id: &'a str,
}

#[derive(Deserialize)]
struct GetTaskResultResponse {
    status: String,
    solution: Option<CaptchaSolution>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptchaSolution {
    #[serde(rename = "gRecaptchaResponse")]
    g_recaptcha_response: String,
}

const CREATE_TASK_URL: &str = "https://api.capsolver.com/createTask";
const GET_TASK_RESULT_URL: &str = "https://api.capsolver.com/getTaskResult";

#[async_trait]
impl CaptchaSolver for CapSolver {
    async fn solve(
        &self,
        website_url: &str,
        website_key: &str,
        task_type: &str,
        page_action: Option<&str>,
        proxy: Option<&String>,
    ) -> Result<String, String> {
        let task = CapSolverTask {
            task_type: task_type,
            website_url: website_url,
            website_key: website_key,
            page_action: page_action,
            proxy: proxy,
        };

        let task_id = create_captcha_task(&self.client_key, task).await?;
        poll_captcha_solution(&self.client_key, task_id).await
    }
}

async fn create_captcha_task(client_key: &str, task: CapSolverTask<'_>) -> Result<String, String> {
    let request_payload = CapSolverRequest {
        client_key: client_key,
        task,
    };

    let response = http_service::send_request(
        CREATE_TASK_URL,
        Method::POST,
        None,
        None,
        Some(&request_payload),
        None,
    ).await?;

    let response_json = http_service::response_to_json(response).await?;
    response_json
        .get("taskId")
        .and_then(|id| id.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to extract taskId".to_string())
}

async fn poll_captcha_solution<'a>(client_key: &'a str, task_id: String) -> Result<String, String> {
    for _ in 0..120 {
        let get_task_result_payload = GetTaskResultRequest {
            client_key: client_key,
            task_id: &task_id.clone(),
        };

        let response = match http_service::send_request(
            GET_TASK_RESULT_URL,
            Method::POST,
            None,
            None,
            Some(&get_task_result_payload),
            None,
        ).await {
            Ok(response) => response,
            Err(_) => continue,
        };

        let response_json: GetTaskResultResponse = match http_service::response_to_json(response).await {
            Ok(response_json) => match serde_json::from_value(response_json) {
                Ok(parsed) => parsed,
                Err(_) => continue,
            },
            Err(_) => continue,
        };

        if response_json.status == "ready" {
            if let Some(solution) = response_json.solution {
                return Ok(solution.g_recaptcha_response);
            }
        } else if response_json.status == "failed" {
            return Err("CAPTCHA task failed".to_string());
        }
    }

    Err("CAPTCHA task timed out".to_string())
}
