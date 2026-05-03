mod ollama;

use anyhow::Result;

use crate::config;

#[derive(Clone)]
pub enum Provider {
    Ollama, // TODO: add different model providers
}

#[derive(Clone)]
pub struct LlmService {
    provider: Provider,
}

impl LlmService {
    pub fn new(provider: Provider) -> Self {
        Self { provider }
    }

    pub async fn ask_git(&self, prompt: &str) -> Result<String> {
        let full_prompt = config::GIT_PROMPT.replace("{prompt}", prompt);

        match self.provider {
            Provider::Ollama => ollama::ask(&full_prompt).await,
        }
    }
}
