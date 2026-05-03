use crate::config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn ask(prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let request = OllamaRequest {
        model: "llama3.1:latest".to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let response = client
        .post(config::PROVIDER_URL)
        .json(&request)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    Ok(response.response.trim().to_string())
}
