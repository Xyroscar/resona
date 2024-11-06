use crate::models::{RequestBody, ResponseBody};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

pub async fn send_request(request: RequestBody) -> Result<ResponseBody, String> {
    let client = reqwest::Client::new();

    // Create headers
    let mut headers = HeaderMap::new();
    for header in request.headers.iter().filter(|h| h.enabled) {
        headers.insert(
            HeaderName::from_str(&header.key).map_err(|e| e.to_string())?,
            HeaderValue::from_str(&header.value).map_err(|e| e.to_string())?,
        );
    }

    let start_time = Instant::now();

    // Build and send request
    let response = match request.method.as_str() {
        "GET" => client.get(&request.url),
        "POST" => client.post(&request.url),
        "PUT" => client.put(&request.url),
        "DELETE" => client.delete(&request.url),
        "PATCH" => client.patch(&request.url),
        _ => return Err("Unsupported HTTP method".to_string()),
    }
    .headers(headers);

    // Add body for methods that support it
    let response = if let Some(body) = request.body {
        response.body(body)
    } else {
        response
    };

    // Send the request
    let response = response
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Process response
    let status = response.status().as_u16();
    let status_text = response.status().to_string();
    
    // Convert response headers
    let headers: HashMap<String, String> = response
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("").to_string(),
            )
        })
        .collect();

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let elapsed = start_time.elapsed().as_millis();

    Ok(ResponseBody {
        status,
        status_text,
        headers,
        body,
        time: elapsed,
    })
} 