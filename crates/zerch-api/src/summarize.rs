use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Debug, Deserialize)]
struct SummarizeRequest {
    text: String,
}

#[derive(Debug, Serialize)]
struct SummarizeResponse {
    summary: String,
}

#[post("/api/summarize")]
async fn summarize(
    req: web::Json<SummarizeRequest>,
    client: web::Data<Client>,
) -> impl Responder {
    // Get API key from environment variable
    let api_key = match env::var("GROQ_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "GROQ_API_KEY not set"}));
        }
    };

    // Build request payload for Groq API (compatible with OpenAI chat format)
    let payload = serde_json::json!({
        "model": "openai/gpt-oss-120b",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant. Summarize the given log text concisely and, if possible, suggest a fix or improvement."},
            {"role": "user", "content": req.text}
        ],
        "temperature": 0.2,
        "max_tokens": 512
    });

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await;

    let resp = match response {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": format!("Request error: {}", e)}));
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": format!("Parse error: {}", e)}));
        }
    };

    // Extract the assistant's message content
    let summary = json["choices"][0]["message"]["content"].as_str().unwrap_or("");

    HttpResponse::Ok().json(SummarizeResponse { summary: summary.to_string() })
}
