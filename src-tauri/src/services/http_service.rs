use reqwest::{Client, Method, Response, Proxy};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use std::collections::HashMap;

pub async fn response_to_json(response: Response) -> Result<Value, String> {
    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response as JSON: {}", e))
}

pub async fn response_to_text(response: Response) -> Result<String, String> {
    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))
}

pub async fn send_request<T: Serialize>(
    url: &str,
    method: Method,
    headers: Option<HeaderMap>,
    query_params: Option<&HashMap<String, String>>,
    body: Option<&T>,
    proxy: Option<Proxy>,
) -> Result<Response, String> {
    let client_builder = Client::builder();
    let client_builder = if let Some(proxy) = proxy {
        client_builder.proxy(proxy)
    } else {
        client_builder
    };

    let client = client_builder
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let mut request_builder = client.request(method, url);
    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    if let Some(query_params) = query_params {
        request_builder = request_builder.query(&query_params);
    }

    if let Some(body) = body {
        request_builder = request_builder.json(body);
    }

    let response = request_builder
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    Ok(response)
}

pub async fn send_request_with_retries<T: Serialize>(
    url: &str,
    headers: HeaderMap,
    proxy: Proxy,
    body: &T,
    max_retries: usize
) -> Result<Response, String> {
    let client = Client::builder()
        .proxy(proxy)
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    for _ in 0..max_retries {
        match client
            .post(url)
            .headers(headers.clone())
            .json(body)
            .send()
            .await
        {
            Ok(response) => return Ok(response),
            Err(e) => println!("Retrying request due to error: {}", e)
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(format!("Failed after {} retries", max_retries))
}
