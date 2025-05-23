use crate::{gh::pulls, git, tui, ai};
use anyhow::{anyhow, Result};

pub async fn pull_create(
    title: Option<String>,
    body: Option<String>,
    base_branch: Option<String>,
    head_branch: Option<String>,
    draft: bool,
    interactive: bool,
    use_ai: bool,
) -> Result<()> {
    let (owner, repo) = git::repo::owner_repo()?;
    let head_branch = head_branch.unwrap_or(git::branch::current()?);

    // Check to make sure a pull request doesn't already exist
    let pull_request = pulls::get_pr_number(&owner, &repo, &head_branch).await?;
    if pull_request.is_some() {
        println!(
            "Pull request url: http://github.com/{}/{}/pull/{}",
            &owner,
            &repo,
            pull_request.unwrap()
        );
        return Err(anyhow!("A pull request already exists for this branch"));
    }

    // If AI is enabled, use it to generate title and body
    let (title, body, draft) = if use_ai {
        println!("Using AI to generate PR title and body...");
        
        // Get the diff and use AI to generate a commit message
        let commit_message = ai::commit::generate().await?;
        
        // The first line of the commit message becomes the title
        let parts: Vec<&str> = commit_message.trim().splitn(2, '\n').collect();
        let ai_title = parts[0].to_string();
        
        // The rest becomes the body (if any)
        let ai_body = if parts.len() > 1 {
            parts[1].trim().to_string()
        } else {
            // If no multiline commit message, generate a more detailed PR description
            // Use commit log instead of diff for PR description
            let commit_log = git::repo::commit_log()?;
            let prompt = ai::prompts::pr_description_prompt(&ai_title, &commit_log);
            ai::ask(&prompt).await?
        };
        
        println!("AI generated title: {}", ai_title);
        (Some(ai_title), Some(ai_body), draft)
    }
    // If interactive mode is enabled, use the TUI to get PR details
    else if interactive {
        let details = tui::pull::create_pull_request()?;
        (Some(details.title), Some(details.body), details.draft)
    } else {
        (title, body, draft)
    };

    // Default to "main" for base branch if not provided
    let base_branch = base_branch.or(Some("main".to_string()));

    match pulls::create_pull_request(
        &owner,
        &repo,
        title.as_deref().unwrap_or(""),
        &head_branch,
        base_branch.as_deref().unwrap_or("main"),
        body.as_deref().unwrap_or(""),
        draft,
    )
    .await
    {
        Ok(pr) => {
            println!("Pull request created successfully!");
            println!("Pull request URL: {}", pr.html_url.unwrap());
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to create pull request: {:?}", e)),
    }
}
