use anyhow::{anyhow, Result};
use git2::Repository;
use std::path::Path;
use std::process::Command;

/// is_repo returns if user is in an active repo
pub fn is_repo() -> Result<bool> {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?;

    let stdout = String::from_utf8(result.stdout)?;
    Ok(stdout.trim().to_string().eq("true"))
}

/// clone will clone a repo locally
pub fn clone(repo: &str, use_ssh: bool) -> Result<()> {
    // Format the URL based on the protocol preference
    let url = if use_ssh {
        format!("git@github.com:{}.git", repo)
    } else {
        format!("https://github.com/{}", repo)
    };

    // Get the repo name from the path
    let repo_name = repo
        .split('/')
        .last()
        .ok_or_else(|| anyhow!("Invalid repository path format"))?;

    // Clone the repository
    Repository::clone(&url, Path::new(repo_name))
        .map_err(|e| anyhow!("Git clone failed: {}", e))?;

    Ok(())
}



/// stage_all is used to stage all Changes
pub fn stage_all() -> Result<()> {
    let result = Command::new("git")
        .arg("stage")
        .arg("-a")
        .arg("./...")
        .output()?;

    if result.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to stage all changes"))
    }
}


/// default_branch returns the default branch
pub fn default_branch() -> Result<String> {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()?;

    let stdout = String::from_utf8(result.stdout)?;
    Ok(stdout.trim().to_string())
}

/// fetch_remote will fetch the remote
pub fn fetch_remote() -> Result<()> {
    let result = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg("--all")
        .arg("--prune")
        .output()?;

    if result.status.success() {
        return Ok(());
    }
    return Err(anyhow!("Failed to fetch remote"));
}

/// pull will pull the latest changes from the remote
pub fn pull(branch: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .output()?;

    if result.status.success() {
        return Ok(());
    }

    return Err(anyhow!("Failed to pull latest changes"));
}