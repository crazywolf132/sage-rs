use crate::{gh::pulls, git};
use anyhow::{anyhow, Result};

pub async fn pull_create(
    title: String,
    body: String,
    base_branch: String,
    head_branch: String,
    draft: bool,
) -> Result<()> {
    let (owner, repo) = git::repo::owner_repo()?;

    // Check to make sure a pull request doesn't already exist
    let pull_request = pulls::get_pr_number(&owner, &repo, &head_branch).await?;
    if pull_request.is_some() {
        return Err(anyhow!("A pull request already exists for this branch"));
    }

    match pulls::create_pull_request(
        &owner,
        &repo,
        &title,
        &head_branch,
        &base_branch,
        &body,
        draft,
    )
    .await
    {
        Ok(pr) => {
            println!("Pull request created successfully!");
            println!("Pull request URL: {}", pr.url);
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to create pull request: {:?}", e)),
    }
}
