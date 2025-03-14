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

/// Gets the PR number associated with a given branch
pub async fn get_pr_number(owner: &str, repo: &str, branch: &str) -> Result<Option<u64>> {
    // Use octocrab's head parameter to filter PRs by branch name directly
    let pull_requests = octocrab::instance()
        .pulls(owner, repo)
        .list()
        .head(branch)  // Filter by head branch name
        .per_page(10)  // We likely only need a few results
        .send()
        .await?
        .take_items();
    
    // Return the number of the first matching PR, if any
    let pr_number = pull_requests.into_iter()
        .next()
        .map(|pr| pr.number);
    
    Ok(pr_number)
}

/// Get the commits for a pull request
pub async fn get_timeline(owner: &str, repo: &str, pr_number: u64) -> Result<Vec<octocrab::models::repos::RepoCommit>> {
    let commits = octocrab::instance()
        .pulls(owner, repo)
        .pr_commits(pr_number)
        .send()
        .await?
        .take_items();
    Ok(commits)
}

/// Get the check runs for a pull request
pub async fn get_checks(owner: &str, repo: &str, pr_number: u64) -> Result<serde_json::Value> {
    // First get the PR to find the head SHA
    let pr = octocrab::instance().pulls(owner, repo).get(pr_number).await?;
    let head_sha = pr.head.sha;
    
    // Use the HTTP API directly
    let json = octocrab::instance()
        .get::<serde_json::Value, _, _>(
            format!("/repos/{}/{}/commits/{}/check-runs", owner, repo, head_sha),
            None::<&()>,
        )
        .await?;
    
    Ok(json)
}