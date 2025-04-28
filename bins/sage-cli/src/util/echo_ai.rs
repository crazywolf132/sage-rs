//! `EchoAI` is a placeholder `AiAssistant` implementation used in local
//! testing so the CLI builds without real OpenAI credentials. It simply
//! returns the prompt truncated to `max_tokens`.

use sage_core::{error::CoreError, port::ai::AiAssistant};

pub struct EchoAI;

impl AiAssistant for EchoAI {
    fn generate_text(&self, prompt: &str, max_tokens: usize) -> Result<String, CoreError> {
        // Just echo the first `max_tokens` words – trivial stub.
        let mut words = prompt.split_whitespace().take(max_tokens).collect::<Vec<_>>();
        if words.is_empty() { words.push("echo") }
        Ok(format!("[stub] {}", words.join(" ")))    }
}
