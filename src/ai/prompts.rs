//! Prompts used for AI-powered features

/// Maximum tokens that can be processed in a single request
pub const MAX_TOKENS: usize = 1_048_576;

/// Prompt for generating commit messages
pub fn commit_message_prompt(diff: &str) -> String {
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
    
    format!("{prefix}{diff}{static_footer}")
}

/// Prompt for generating pull request descriptions
pub fn pr_description_prompt(title: &str, commit_log: &str) -> String {
    format!(
        r#"You are writing a GitHub pull request description for a change with the title: "{}".
        
        Here's information about the commits in this PR:
        ```
        {}
        ```
        
        Follow these guidelines for an effective PR description:
        
        1. Start with a brief summary of what this PR achieves (1-2 sentences).
        2. Explain the problem this PR solves and why it's important.
        3. Highlight key changes or new features introduced.
        4. If applicable, mention any testing performed or areas that would benefit from extra review.
        5. If there are any breaking changes, dependencies, or deployment considerations, note them.
        
        Format your description professionally, using proper Markdown formatting with headers and lists where appropriate.
        Be concise yet thorough - aim for clarity and completeness.
        
        Your response should ONLY include the PR description text, no additional explanations or comments."#,
        title,
        commit_log
    )
}
