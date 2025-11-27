use std::time::Instant;

use reqwest::{header::HeaderMap, Client, Method};

use super::types::{HttpRequest, HttpResponse, HttpResponseHeader};

pub async fn execute_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let client = Client::new();
    let start = Instant::now();

    // Parse method
    let method = match request.method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "PATCH" => Method::PATCH,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        _ => return Err(format!("Unsupported HTTP method: {}", request.method)),
    };

    // Build URL with query params
    let mut url = request.url.clone();
    let enabled_params: Vec<_> = request
        .params
        .iter()
        .filter(|p| p.enabled && !p.key.is_empty())
        .collect();

    if !enabled_params.is_empty() {
        let query_string: String = enabled_params
            .iter()
            .map(|p| format!("{}={}", urlencoding::encode(&p.key), urlencoding::encode(&p.value)))
            .collect::<Vec<_>>()
            .join("&");

        if url.contains('?') {
            url = format!("{}&{}", url, query_string);
        } else {
            url = format!("{}?{}", url, query_string);
        }
    }

    // Build headers
    let mut headers = HeaderMap::new();
    for header in &request.headers {
        if header.enabled && !header.key.is_empty() {
            if let (Ok(name), Ok(value)) = (
                header.key.parse::<reqwest::header::HeaderName>(),
                header.value.parse::<reqwest::header::HeaderValue>(),
            ) {
                headers.insert(name, value);
            }
        }
    }

    // Build request
    let mut req_builder = client.request(method, &url).headers(headers);

    // Add body based on body type
    match request.body_type.as_str() {
        "json" => {
            req_builder = req_builder
                .header("Content-Type", "application/json")
                .body(request.body.clone());
        }
        "xml" => {
            req_builder = req_builder
                .header("Content-Type", "application/xml")
                .body(request.body.clone());
        }
        "text" => {
            req_builder = req_builder
                .header("Content-Type", "text/plain")
                .body(request.body.clone());
        }
        "html" => {
            req_builder = req_builder
                .header("Content-Type", "text/html")
                .body(request.body.clone());
        }
        "x-www-form-urlencoded" => {
            let enabled_form: Vec<_> = request
                .form_data
                .iter()
                .filter(|f| f.enabled && !f.key.is_empty())
                .collect();

            let form_string: String = enabled_form
                .iter()
                .map(|f| format!("{}={}", urlencoding::encode(&f.key), urlencoding::encode(&f.value)))
                .collect::<Vec<_>>()
                .join("&");

            req_builder = req_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(form_string);
        }
        "form-data" => {
            let mut form = reqwest::multipart::Form::new();
            for item in &request.form_data {
                if item.enabled && !item.key.is_empty() {
                    if item.item_type == "file" {
                        // For file uploads, read the file
                        match std::fs::read(&item.value) {
                            Ok(contents) => {
                                let file_name = std::path::Path::new(&item.value)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("file")
                                    .to_string();
                                let part = reqwest::multipart::Part::bytes(contents)
                                    .file_name(file_name);
                                form = form.part(item.key.clone(), part);
                            }
                            Err(e) => {
                                return Err(format!("Failed to read file {}: {}", item.value, e));
                            }
                        }
                    } else {
                        form = form.text(item.key.clone(), item.value.clone());
                    }
                }
            }
            req_builder = req_builder.multipart(form);
        }
        _ => {
            // "none" or unknown - no body
        }
    }

    // Execute request
    let response = req_builder
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let elapsed = start.elapsed();
    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown")
        .to_string();

    // Collect headers
    let response_headers: Vec<HttpResponseHeader> = response
        .headers()
        .iter()
        .map(|(k, v)| HttpResponseHeader {
            key: k.to_string(),
            value: v.to_str().unwrap_or("").to_string(),
        })
        .collect();

    // Get body
    let body_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let size_bytes = body_bytes.len();
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    Ok(HttpResponse {
        status,
        status_text,
        headers: response_headers,
        body,
        time_ms: elapsed.as_millis() as u64,
        size_bytes,
    })
}
