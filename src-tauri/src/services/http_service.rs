use reqwest::{Client, Method, Response, Proxy};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use std::collections::HashMap;

/// Converts a HTTP response to a JSON value.
///
/// # Arguments
/// - `response`: The `Response` object to be converted.
///
/// # Returns
/// A `Result` with a `Value` on success or an error message on failure.
pub async fn response_to_json(response: Response) -> Result<Value, String> {
    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response as JSON: {}", e))
}

/// Converts a HTTP response to a text string.
///
/// # Arguments
/// - `response`: The `Response` object to be converted.
///
/// # Returns
/// A `Result` with a `String` on success or an error message on failure.
pub async fn response_to_text(response: Response) -> Result<String, String> {
    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))
}

/// Sends an HTTP request with the given parameters.
///
/// # Arguments
/// - `url`: The target URL for the request.
/// - `method`: The HTTP method to use.
/// - `headers`: Optional headers for the request.
/// - `query_params`: Optional query parameters for the request.
/// - `body`: Optional body for the request.
/// - `proxy`: Optional proxy for the request.
///
/// # Returns
/// A `Result` with a `Response` on success or an error message on failure.
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

/// Sends an HTTP request with retries in case of failure.
///
/// # Arguments
/// - `url`: The target URL for the request.
/// - `headers`: Headers to include in the request.
/// - `proxy`: A proxy to route the request through.
/// - `body`: The body of the request.
/// - `max_retries`: The maximum number of retries in case of failure.
///
/// # Returns
/// A `Result` with a `Response` on success or an error message on failure.
///
/// # Behavior
/// - This function retries the request up to `max_retries` times with a 1-second delay between attempts.
/// - Returns the first successful response or an error if all retries fail.
pub async fn send_request_with_retries<T: Serialize>(
    url: &str,
    headers: HeaderMap,
    proxy: Proxy,
    body: &T,
    max_retries: usize
) -> Result<Response, String> {
    for _ in 0..max_retries {
        match send_request(
            url,
            Method::POST,
            Some(headers.clone()),
            None,
            Some(body),
            Some(proxy.clone())).await
        {
            Ok(response) => return Ok(response),
            Err(e) => println!("Retrying request due to error: {}", e)
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(format!("Failed after {} retries", max_retries))
}
