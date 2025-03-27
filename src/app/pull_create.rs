use crate::{gh::pulls, git, tui};
use anyhow::{anyhow, Result};

pub async fn pull_create(
    title: Option<String>,
    body: Option<String>,
    base_branch: Option<String>,
    head_branch: Option<String>,
    draft: bool,
    interactive: bool,
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

    // If interactive mode is enabled, use the TUI to get PR details
    let (title, body, draft) = if interactive {
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
