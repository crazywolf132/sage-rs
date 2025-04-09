use anyhow::Result;
use crate::{git, ai::prompts};

pub async fn generate() -> Result<String> {
    let max_diff_length = prompts::MAX_TOKENS - prompts::commit_message_prompt("").len();
    let mut diff = git::repo::diff()?;

    if diff.len() > max_diff_length {
        diff = diff.chars().take(max_diff_length).collect::<String>() + "\n[diff truncated]";
    }

    let prompt = prompts::commit_message_prompt(&diff);
    let res = super::ask(&prompt).await?;
    
    // Remove surrounding backticks if present
    let res = res.trim();
    let res = if res.starts_with("```") && res.ends_with("```") {
        res.trim_start_matches("```").trim_end_matches("```").trim().to_string()
    } else {
        res.to_string()
    };
    
    Ok(res.to_string())
}