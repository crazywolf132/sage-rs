use anyhow::Result;
use crate::git;

const MAX_TOKENS: usize = 1_048_576;

pub async fn generate() -> Result<String> {
    let prefix = r#"
    You are a helpful git commit message generator. Your task is to analyze the following code changes and generate a clear, meaningful commit message that follows the Conventional Commits specification.

Guidelines:
1. Use one of these types:
   - feat: A new feature
   - fix: A bug fix
   - docs: Documentation changes
   - style: Code style changes (formatting, missing semi-colons, etc)
   - refactor: Code changes that neither fix a bug nor add a feature
   - test: Adding or modifying tests
   - ci: Changes to CI/CD configuration and scripts
   - chore: Changes to build process or auxiliary tools

2. Format: <type>: <description>
   Examples:
   - feat: add user authentication system
   - fix: resolve null pointer in data processing
   - ci: update GitHub Actions workflow

3. Analyze the diff carefully:
   - Look for function/method additions or modifications
   - Identify bug fixes from error handling changes
   - Note any test additions or modifications
   - Consider impact on existing functionality
   - Changes in .github/workflows/ directory should use 'ci' type
   - Changes to CI/CD pipeline configurations should use 'ci' type

4. Keep the message:
   - Concise but informative (ideally under 72 characters)
   - Focused on WHAT changed and WHY
   - In imperative mood ("add" not "added")
   - Without unnecessary technical details

Code changes to analyze:
    "#;

    let static_footer = "Respond with ONLY the commit message, no additional text or formatting.";
    let max_diff_length = MAX_TOKENS - (prefix.len() + static_footer.len());
    let mut diff = git::repo::diff()?;

    if diff.len() > max_diff_length {
        diff = diff.chars().take(max_diff_length).collect::<String>() + "\n[diff truncated]";
    }

    let res = super::ask(&format!("{prefix}{diff}{static_footer}")).await?; 
    Ok(res)
}