use anyhow::{Context, Result, anyhow};
use std::process::Command;

/// current_branch returns the current branch name
pub fn current() -> Result<String> {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    let branch_name = String::from_utf8(result?.stdout)?;

    Ok(branch_name.trim().to_string())
}

/// switch switches a branch, and will create it if required
pub fn switch(branch_name: String, create: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("switch");
    if create {
        cmd.arg("-c");
    }
    
    // This captures the output instead of displaying it
    let output = cmd.arg(branch_name)
        .output()
        .expect("failed to switch branch");
    
    if !output.status.success() {
        return Err(anyhow!("Failed to switch branch: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

/// list -- returns a list of the branches locally
pub fn list() -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()
        .context("failed to list branches")?;

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout
        .lines()
        .map(String::from)
        .map(|s| s.trim().to_string())
        .collect())
}

/// Get a struct containing information about a branch including its upstream, ahead and behind counts
#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub upstream: Option<String>,
    pub ahead_count: usize,
    pub behind_count: usize,
    pub is_current: bool,
}

/// list_with_info -- returns a list of branches with additional information
pub fn list_with_info() -> Result<Vec<BranchInfo>> {
    // Get the current branch first
    let current_branch = current()?;
    
    // Get all branches
    let branches = list()?;
    
    // Create a result vector
    let mut result = Vec::with_capacity(branches.len());
    
    for branch in branches {
        let (upstream, ahead, behind) = get_branch_tracking_info(&branch)?;
        
        result.push(BranchInfo {
            name: branch.clone(),
            upstream,
            ahead_count: ahead,
            behind_count: behind,
            is_current: branch == current_branch,
        });
    }
    
    Ok(result)
}

/// Get tracking information for a specific branch
/// Returns a tuple of (upstream_branch, ahead_count, behind_count)
fn get_branch_tracking_info(branch: &str) -> Result<(Option<String>, usize, usize)> {
    // Get the upstream branch
    let upstream_output = Command::new("git")
        .args(["for-each-ref", "--format=%(upstream:short)", &format!("refs/heads/{}", branch)])
        .output()
        .context("Failed to get upstream branch")?;
    
    let upstream_str = String::from_utf8(upstream_output.stdout)?
        .trim()
        .to_string();
    
    // If there's no upstream branch, return early
    if upstream_str.is_empty() {
        return Ok((None, 0, 0));
    }
    
    // Now get ahead/behind counts
    let rev_list_args = format!("{}...{}", upstream_str, branch);
    let count_output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", &rev_list_args])
        .output()
        .context("Failed to get ahead/behind counts")?;
    
    if !count_output.status.success() {
        return Ok((Some(upstream_str), 0, 0));
    }
    
    // Parse the output
    let counts = String::from_utf8(count_output.stdout)?
        .trim()
        .to_string();
    
    let parts: Vec<&str> = counts.split_whitespace().collect();
    let behind = if parts.len() > 0 { parts[0].parse().unwrap_or(0) } else { 0 };
    let ahead = if parts.len() > 1 { parts[1].parse().unwrap_or(0) } else { 0 };
    
    Ok((Some(upstream_str), ahead, behind))
}

/// push will push the current branch to remote
pub fn push(branch_name: &str, force: bool) -> Result<()> {

    let mut cmd = Command::new("git");
    cmd.arg("push");
    cmd.arg("--set-upstream");
    cmd.arg("origin");
    cmd.arg(branch_name);

    if force {
        cmd.arg("--force-with-lease");
    }

    let result = cmd.output()?;
    if result.status.success() {
        return Ok(());
    }

    return Err(anyhow!("Failed to push branch"));
}


/// exists returns if a branch exists
pub fn exists(branch_name: &str) -> bool {
    let branches = list().unwrap_or(vec![]);
    branches.iter().any(|b| b == branch_name)
}