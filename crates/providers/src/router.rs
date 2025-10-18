use anyhow::Result;
use shared::agent_api::ChatMessage;
use crate::gemini::GeminiClient;
use crate::ollama::OllamaClient;

pub struct ProviderRouter {
    local_model: String,
    enable_gemini: bool,
}

impl ProviderRouter {
    pub fn new(local_model: String, enable_gemini: bool) -> Self {
        Self { local_model, enable_gemini }
    }

    pub async fn generate(&self, messages: Vec<ChatMessage>) -> Result<String> {
        // Try local first
        let ollama = OllamaClient::new(self.local_model.clone());
        match ollama.generate(messages.clone()).await {
            Ok(s) => return Ok(s),
            Err(e) => {
                if !self.enable_gemini { return Err(e); }
            }
        }
        // Fallback to Gemini if enabled and configured
        let gemini = GeminiClient::new("gemini-1.5-flash");
        match gemini {
            Ok(g) => g.generate(messages).await,
            Err(e) => Err(e),
        }
    }
}
