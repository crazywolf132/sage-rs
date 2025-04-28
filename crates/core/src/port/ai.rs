use crate::error::CoreError;

pub trait AiAssistant {
    fn generate_text(&self, prompt: &str, max_tokens: usize) -> Result<String, CoreError>;
}
