use octocrab::models::pulls::PullRequest;
use anyhow::Result;
use crate::gh;

/// Gets a single pull request for a given repository
pub async fn get_pull_request(owner: &str, repo: &str, pr_number: u64) -> Result<PullRequest> {
    let pull_request = gh::get_instance().pulls(owner, repo).get(pr_number).await?;
    Ok(pull_request)
}

/// Lists all pull requests for a given repository
pub async fn list_pull_requests(owner: &str, repo: &str) -> Result<Vec<PullRequest>> {
    let pull_requests = gh::get_instance().pulls(owner, repo).list().per_page(100).page(1u32).send().await?.take_items();
    Ok(pull_requests)
}

/// Creates a new pull request for a given repository
pub async fn create_pull_request(owner: &str, repo: &str, title: &str, head: &str, base: &str, body: &str) -> Result<PullRequest> {
    let pull_request = gh::get_instance().pulls(owner, repo).create(title, head, base).body(body).send().await?;
    Ok(pull_request)
}

/// Gets the PR number associated with a given branch
pub async fn get_pr_number(owner: &str, repo: &str, branch: &str) -> Result<Option<u64>> {
    // Use octocrab's head parameter to filter PRs by branch name directly
    let pull_requests = gh::get_instance()
        .pulls(owner, repo)
        .list()
        .head(branch)  // Filter by head branch name
        .per_page(10)  // We likely only need a few results
        .send()
        .await?
        .take_items();

    // If we find a PR with the given branch, return its number
    if let Some(pr) = pull_requests.first() {
        return Ok(Some(pr.number));
    }

    Ok(None)
}

/// Gets the timeline of a pull request (list of commits)
pub async fn get_timeline(owner: &str, repo: &str, pr_number: u64) -> Result<Vec<octocrab::models::repos::RepoCommit>> {
    // Get commits for the PR using the correct endpoint
    let commits = gh::get_instance()
        .pulls(owner, repo)
        .pr_commits(pr_number)
        .per_page(10) // Limit to 10 most recent commits
        .send()
        .await?
        .take_items();
    
    Ok(commits)
}

/// Gets the checks for a pull request
pub async fn get_checks(owner: &str, repo: &str, pr_number: u64) -> Result<serde_json::Value> {
    // First, get the pull request to get the head SHA
    let pr = gh::get_instance().pulls(owner, repo).get(pr_number).await?;
    let head_sha = pr.head.sha;
    
    // Use the http client directly to call the check-runs endpoint
    let route = format!("/repos/{}/{}/commits/{}/check-runs", owner, repo, head_sha);
    let response = gh::get_instance()
        .get::<serde_json::Value, _, ()>(&route, None)
        .await?;
    
    Ok(response)
}