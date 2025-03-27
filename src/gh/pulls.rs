use crate::errors::GitHubError;
use crate::gh;
use anyhow::Result;
use octocrab::models::pulls::PullRequest;

/// Maps octocrab errors to our custom GitHubError types
fn map_github_error(err: octocrab::Error) -> anyhow::Error {
    // Convert the error to a string to check for specific error conditions
    let err_string = err.to_string();

    if err_string.contains("401") || err_string.contains("Unauthorized") {
        GitHubError::AuthenticationError.into()
    } else if err_string.contains("404") || err_string.contains("Not Found") {
        GitHubError::NotFound("Pull request or repository not found".to_string()).into()
    } else if err_string.contains("403") || err_string.contains("rate limit") {
        GitHubError::RateLimitExceeded.into()
    } else {
        GitHubError::RequestError(format!("GitHub API error: {}", err)).into()
    }
}

/// Gets a single pull request for a given repository
pub async fn get_pull_request(owner: &str, repo: &str, pr_number: u64) -> Result<PullRequest> {
    gh::get_instance()
        .pulls(owner, repo)
        .get(pr_number)
        .await
        .map_err(map_github_error)
}

/// Lists all pull requests for a given repository
pub async fn list_pull_requests(owner: &str, repo: &str) -> Result<Vec<PullRequest>> {
    gh::get_instance()
        .pulls(owner, repo)
        .list()
        .per_page(100)
        .page(1u32)
        .send()
        .await
        .map_err(map_github_error)
        .map(|mut page| page.take_items())
}

/// Creates a new pull request for a given repository
pub async fn create_pull_request(
    owner: &str,
    repo: &str,
    title: &str,
    head: &str,
    base: &str,
    body: &str,
    draft: bool,
) -> Result<PullRequest> {
    gh::get_instance()
        .pulls(owner, repo)
        .create(title, head, base)
        .body(body)
        .draft(Some(draft))
        .send()
        .await
        .map_err(map_github_error)
}

/// Gets the PR number associated with a given branch
pub async fn get_pr_number(owner: &str, repo: &str, branch: &str) -> Result<Option<u64>> {
    // Use octocrab's head parameter to filter PRs by branch name directly
    let pull_requests = gh::get_instance()
        .pulls(owner, repo)
        .list()
        .head(branch) // Filter by head branch name
        .per_page(10) // We likely only need a few results
        .send()
        .await
        .map_err(map_github_error)?
        .take_items();

    // If we find a PR with the given branch, return its number
    if let Some(pr) = pull_requests.first() {
        return Ok(Some(pr.number));
    }

    Ok(None)
}

/// Gets the timeline of a pull request (list of commits)
pub async fn get_timeline(
    owner: &str,
    repo: &str,
    pr_number: u64,
) -> Result<Vec<octocrab::models::repos::RepoCommit>> {
    // Get commits for the PR using the correct endpoint
    let commits = gh::get_instance()
        .pulls(owner, repo)
        .pr_commits(pr_number)
        .per_page(10) // Limit to 10 most recent commits
        .send()
        .await
        .map_err(map_github_error)?
        .take_items();

    Ok(commits)
}

/// Gets the checks for a pull request
pub async fn get_checks(owner: &str, repo: &str, pr_number: u64) -> Result<serde_json::Value> {
    // First, get the pull request to get the head SHA
    let pr = gh::get_instance()
        .pulls(owner, repo)
        .get(pr_number)
        .await
        .map_err(map_github_error)?;

    let head_sha = pr.head.sha;

    // Use the http client directly to call the check-runs endpoint
    let route = format!("/repos/{}/{}/commits/{}/check-runs", owner, repo, head_sha);
    let response = gh::get_instance()
        .get::<serde_json::Value, _, ()>(&route, None)
        .await
        .map_err(map_github_error)?;

    Ok(response)
}

