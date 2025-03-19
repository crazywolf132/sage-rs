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
        .arg("--all")
        .arg("--prune")
        .output()?;

    if result.status.success() {
        return Ok(());
    }
    return Err(anyhow!("Failed to fetch remote"));
}

/// pull will pull the latest changes from the remote
pub fn pull(branch: &str, fast_forward: bool) -> Result<()> {
    let mut cmd = Command::new("git");
        cmd.arg("pull");
        cmd.arg("origin");
        cmd.arg(branch);

    if fast_forward {
        cmd.arg("--ff-only");
    }
    let result = cmd.output()?;

    if result.status.success() {
        return Ok(());
    }

    return Err(anyhow!("Failed to pull latest changes. {}", String::from_utf8(result.stderr)?));
}

/// get the owner and repo name from the remote URL
pub fn owner_repo() -> Result<(String, String)> {
    let result = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()?;

    
    // The repo url could be SSH or it could be HTTPS
    // We are going to handle both cases here.

    let remote_url = String::from_utf8(result.stdout)?.trim().to_string();
    if remote_url.starts_with("git@github.com:") {
        let parts = remote_url.trim_start_matches("git@github.com:")
            .trim_end_matches(".git")
            .split('/')
            .collect::<Vec<_>>();

        if parts.len() >= 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }

    // If we are here... we have an HTTPS URL
    let parts = remote_url.trim_start_matches("https://github.com/")
        .trim_end_matches(".git")
        .split("/")
        .collect::<Vec<_>>();

    if parts.len() >= 2 {
        return Ok((parts[0].to_string(), parts[1].to_string()));
    }

    unreachable!("Invalid remote URL");
}


/// fetch with a specific refspec
pub fn fetch(refspec: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(refspec)
        .output()?;

    if result.status.success() {
        return Ok(());
    }
    Ok(())
}