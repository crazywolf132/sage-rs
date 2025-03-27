use crate::{gh::pulls, git};
use anyhow::{anyhow, Result};

pub async fn pull_create(
    title: String,
    body: String,
    base_branch: String,
    head_branch: Option<String>,
    draft: bool,
) -> Result<()> {
    let (owner, repo) = git::repo::owner_repo()?;
    let head_branch = head_branch.unwrap_or(git::branch::current()?);

    println!("Title: {}", &title);
    println!("Body: {}", &body);
    println!("Head branch: {}", &head_branch);
    println!("Base branch: {}", &base_branch);
    println!("Draft: {}", &draft);

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
            println!("Pull request URL: {}", pr.html_url.unwrap());
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to create pull request: {:?}", e)),
    }
}
