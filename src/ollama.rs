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

pub async fn ask_ollama(prompt: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let request = OllamaRequest {
        model: "llama3.1:latest".to_string(),
        prompt: format!(
            "You are a git expert. Convert this natural language request into a sequence of git shell commands. Return ONLY the commands, one per line, no explanation, no markdown, no backticks.
            Request: {}",
            prompt
        ),
        stream: false,
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    Ok(response.response.trim().to_string())
}
