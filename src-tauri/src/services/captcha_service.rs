use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapSolverTask {
    #[serde(rename = "type")]
    task_type: String,
    #[serde(rename = "websiteURL")]
    website_url: String,
    website_key: String,
    page_action: Option<String>,
    proxy: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapSolverRequest {
    client_key: String,
    task: CapSolverTask,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetTaskResultRequest {
    client_key: String,
    task_id: String,
}

#[derive(Deserialize)]
struct GetTaskResultResponse {
    status: String,
    solution: Option<CaptchaSolution>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptchaSolution {
    g_recaptcha_response: String,
}

const CREATE_TASK_URL: &str = "https://api.capsolver.com/createTask";
const GET_TASK_RESULT_URL: &str = "https://api.capsolver.com/getTaskResult";

pub async fn solve_captcha(
    client_key: String,
    website_url: &str,
    website_key: &str,
    task_type: &str,
    page_action: Option<&str>,
    proxy: Option<String>,
) -> Result<String, String> {
    let task = CapSolverTask {
        task_type: task_type.to_string(),
        website_url: website_url.to_string(),
        website_key: website_key.to_string(),
        page_action: page_action.map(|p| p.to_string()),
        proxy: proxy.map(|p| p.to_string()),
    };

    let task_id = create_captcha_task(&client_key, task).await?;
    poll_captcha_solution(&client_key, task_id).await
}

async fn create_captcha_task(client_key: &str, task: CapSolverTask) -> Result<String, String> {
    let request_payload = CapSolverRequest {
        client_key: client_key.to_string(),
        task,
    };

    let client = Client::new();
    let response = client
        .post(CREATE_TASK_URL)
        .json(&request_payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send createTask request: {}", e))?;

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse createTask response: {}", e))?;

    response_json
        .get("taskId")
        .and_then(|id| id.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to extract taskId".to_string())
}

async fn poll_captcha_solution(client_key: &str, task_id: String) -> Result<String, String> {
    let client = Client::new();
    for _ in 0..120 {
        let get_task_result_payload = GetTaskResultRequest {
            client_key: client_key.to_string(),
            task_id: task_id.clone(),
        };

        let result_response = client
            .post(GET_TASK_RESULT_URL)
            .json(&get_task_result_payload)
            .send()
            .await
            .map_err(|e| format!("Failed to send getTaskResult request: {}", e))?;

        let result_json: GetTaskResultResponse = result_response
            .json()
            .await
            .map_err(|e| format!("Failed to parse getTaskResult response: {}", e))?;

        if result_json.status == "ready" {
            if let Some(solution) = result_json.solution {
                return Ok(solution.g_recaptcha_response);
            }
        } else if result_json.status == "failed" {
            return Err("CAPTCHA task failed".to_string());
        }
    }

    Err("CAPTCHA task timed out".to_string())
}
