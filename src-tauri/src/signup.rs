use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: Value,
}

#[derive(Serialize)]
struct CapsolverRequest {
    clientKey: String,
    task: CapsolverTask,
}

#[derive(Serialize)]
struct CapsolverTask {
    #[serde(rename = "type")]
    task_type: String,
    websiteURL: String,
    websiteKey: String,
    pageAction: String,
    proxy: Option<String>,
}

#[derive(Debug, Serialize)]
struct GetTaskResultRequest {
    clientKey: String,
    taskId: String,
}

#[derive(Debug, Deserialize)]
struct GetTaskResultResponse {
    status: String,
    solution: Option<Solution>,
}

#[derive(Debug, Deserialize)]
struct Solution {
    gRecaptchaResponse: String,
}

#[tauri::command]
pub async fn sabrina_hallenstadion(proxy_group: String, mode: String) -> Result<(), String> {
    println!("Starting signups with proxy group: {:?}, mode: {:?}", proxy_group, mode);

    let captcha_solution = solve_captcha(Some(proxy_group))
        .await
        .map_err(|e| format!("Captcha solving failed: {}", e))?;
    println!("Captcha solution received: {}", captcha_solution);

    let url = "https://laylo.com/api/graphql";

    // GraphQL query string
    let query = r#"
        mutation (
          $productId: ID!
          $userId: String
          $email: String
          $dropDate: Float
          $fingerprintId: String
          $phoneNumber: String
          $optIn: Boolean
          $referrer: String
          $captcha: String
          $instagramId: String
        ) {
          purchaseProduct(
            productId: $productId
            userId: $userId
            email: $email
            phoneNumber: $phoneNumber
            dropDate: $dropDate
            fingerprintId: $fingerprintId
            optIn: $optIn
            referrer: $referrer
            captcha: $captcha
            instagramId: $instagramId
          )
        }
    "#;

    // JSON object for variables
    let variables = serde_json::json!({
        "dropDate": 1733386800000i64,
        "fingerprintId": "3542ca6347494934b9e7be13fa3922e7",
        "productId": "bdd68f6b-dd1c-4551-89d7-c168f8d6c3d0",
        "optIn": false,
        "email": "elias@jamee.se",
        "referrer": "https://store.sabrinacarpenter.com/pages/tour",
        "captcha": captcha_solution,
    });

    // Create a GraphQL request payload
    let graphql_request = GraphQLRequest {
        query: query.to_string(),
        variables,
    };

    println!("GraphQL request: {:?}", graphql_request);

    // Send the HTTP POST request
    let client = Client::new();
    let response = client
        .post(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36")
        .header("Referer", "https://embed.laylo.com") // Updated referer
        .header("Origin", "https://embed.laylo.com")  // Ensures domain matching
        .json(&graphql_request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    println!("Response: {}", response_text);

    Ok(())
}

async fn solve_captcha(proxy: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let create_task_url = "https://api.capsolver.com/createTask";
    let get_task_result_url = "https://api.capsolver.com/getTaskResult";

    let client_key = "CAP-50C1E71AF959204A1440C4E490AA036E".to_string();

    // Capsolver task payload
    let task = CapsolverTask {
        task_type: "ReCaptchaV3TaskProxyLess".to_string(),
        websiteURL: "https://embed.laylo.com".to_string(), // Use the decoded domain
        websiteKey: "6LfaRWApAAAAAPvWsG2tsIhBCLEdXyz_EUQtQily".to_string(),
        pageAction: "create_customer".to_string(), // Replace with verified action
        proxy,
    };

    let request_payload = CapsolverRequest {
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
    println!("CreateTask response: {:?}", response_json);

    let task_id = response_json
        .get("taskId")
        .and_then(|id| id.as_str())
        .ok_or("Failed to extract taskId")?;
    println!("Task ID: {}", task_id);

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
        println!("Polling response: {:?}", result_json);

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
