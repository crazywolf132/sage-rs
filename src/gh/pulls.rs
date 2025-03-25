use octocrab::models::pulls::PullRequest;
use anyhow::Result;

/// Gets a single pull request for a given repository
pub async fn get_pull_request(owner: &str, repo: &str, pr_number: u64) -> Result<PullRequest> {
    let pull_request = octocrab::instance().pulls(owner, repo).get(pr_number).await.unwrap();
    Ok(pull_request)
}

/// Lists all pull requests for a given repository
pub async fn list_pull_requests(owner: &str, repo: &str) -> Result<Vec<PullRequest>> {
    let pull_requests = octocrab::instance().pulls(owner, repo).list().per_page(100).page(1u32).send().await?.take_items();
    Ok(pull_requests)
}

/// Creates a new pull request for a given repository
pub async fn create_pull_request(owner: &str, repo: &str, title: &str, head: &str, base: &str, body: &str) -> Result<PullRequest> {
    let pull_request = octocrab::instance().pulls(owner, repo).create(title, head, base).body(body).send().await?;
    Ok(pull_request)
}