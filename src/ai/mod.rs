use std::env;
use anyhow::{Result, Context, anyhow};
use openai_api_rs::v1::{api::OpenAIClient, chat_completion::{self, ChatCompletionRequest}, common::GPT4_O};

/// Asks the AI with a prompt
pub async fn ask(prompt: &str) -> Result<String> {
    // Get API key
    let api_key = env::var("OPENAI_API_KEY")
        .context("Failed to get OPENAI_API_KEY environment variable")?;
    
    // Build client
    let mut client = OpenAIClient::builder()
        .with_api_key(&api_key)
        .build()
        .expect("Failed to build OpenAI client");
    
    // Create request
    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(prompt)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }
        ],
    );

    // Get response
    let result = client.chat_completion(req).await
        .context("Failed to get chat completion")?;
    
    // Ensure we have choices
    if result.choices.is_empty() {
        return Err(anyhow!("No choices returned from API"));
    }

    // Extract and return content
    match &result.choices[0].message.content {
        Some(content) => Ok(content.to_string()),
        None => Err(anyhow!("No content in the response message")),
    }
}