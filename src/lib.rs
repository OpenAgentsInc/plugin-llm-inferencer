use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
struct InferenceRequest {
    model_name: String,
    input_content: String,
    api_key: String,
}

#[plugin_fn]
pub fn inference(request_json: String) -> FnResult<String> {
    // Parse the JSON string into the InferenceRequest struct
    // Note: Simplified and not handling parse errors
    let request: InferenceRequest = serde_json::from_str(&request_json).unwrap();

    let body = json!({
        "model": request.model_name,
        "messages": [{"role": "user", "content": request.input_content}],
        "temperature": 0.7,
    }).to_string();

    // Create and populate the headers
    let mut headers = BTreeMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Authorization".to_string(), format!("Bearer {}", request.api_key));

    // Construct the HttpRequest
    let mut req = HttpRequest::new("https://api.openai.com/v1/chat/completions")
        .with_method("POST");

    // Add headers
    for (key, value) in headers {
        req = req.with_header(key, value);
    }

    // Send the HTTP request and get the response
    // Note: Simplified and not handling request errors
    let res = http::request(&req, Some(body.into_bytes())).unwrap();

    // Convert the response body to a String
    // Note: Simplified and not handling UTF-8 decoding errors
    let response_body = String::from_utf8(res.body().clone()).unwrap();

    Ok(response_body)
}

