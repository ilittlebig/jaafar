use reqwest::Client;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct CapSolverTask {
    #[serde(rename = "type")]
    taskType: String,
    websiteURL: String,
    websiteKey: String,
    pageAction: Option<String>,
    proxy: Option<String>,
}

#[derive(Debug, Serialize)]
struct CapSolverRequest {
    clientKey: String,
    task: CapSolverTask,
}

#[derive(Debug, Serialize)]
struct GetTaskResultRequest {
    clientKey: String,
    taskId: String,
}

#[derive(Debug, Deserialize)]
struct GetTaskResultResponse {
    status: String,
    solution: Option<CaptchaSolution>,
}

#[derive(Debug, Deserialize)]
struct CaptchaSolution {
    gRecaptchaResponse: String,
}

pub async fn solve_captcha(
    client_key: String,
    website_url: &str,
    website_key: &str,
    task_type: &str,
    page_action: Option<&str>,
    proxy: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let create_task_url = "https://api.capsolver.com/createTask";
    let get_task_result_url = "https://api.capsolver.com/getTaskResult";

    let task = CapSolverTask {
        taskType: task_type.to_string(),
        websiteURL: website_url.to_string(),
        websiteKey: website_key.to_string(),
        pageAction: page_action.map(|p| p.to_string()),
        proxy: proxy.map(|p| p.to_string()),
    };

    let request_payload = CapSolverRequest {
        clientKey: client_key.clone(),
        task,
    };

    let client = Client::new();
    let response = client
        .post(create_task_url)
        .json(&request_payload)
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    let task_id = response_json
        .get("taskId")
        .and_then(|id| id.as_str())
        .ok_or("Failed to extract taskId")?;

    for _ in 0..120 {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let get_task_result_payload = GetTaskResultRequest {
            clientKey: client_key.clone(),
            taskId: task_id.to_string(),
        };

        let result_response = client
            .post(get_task_result_url)
            .json(&get_task_result_payload)
            .send()
            .await?;

        let result_json: GetTaskResultResponse = result_response.json().await?;
        if result_json.status == "ready" {
            if let Some(solution) = result_json.solution {
                return Ok(solution.gRecaptchaResponse);
            }
        } else if result_json.status == "failed" {
            return Err("CAPTCHA task failed".into());
        }
    }

    Err("CAPTCHA task timed out".into())
}
