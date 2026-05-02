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
            "You are a git expert. Convert this natural language request into git shell commands.
STRICT RULES:
- Return ONLY the commands that should be run, one per line
- NO explanations, NO comments, NO alternatives
- NO markdown, NO backticks
- If multiple approaches exist, pick the best one silently
- Every line must be a valid executable shell command starting with 'git'
- If you return more than one approach, you have failed. One approach only.
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
