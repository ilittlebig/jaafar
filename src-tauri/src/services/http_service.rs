use reqwest::{Client, Response, Proxy};
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::time::Duration;

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
