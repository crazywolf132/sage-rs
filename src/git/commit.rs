use anyhow::{anyhow, Result};
use std::process::Command;

/// list_branches returns a list of all local branches
pub fn list_branches() -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()
        .expect("Could not list commits");

    let stdout = String::from_utf8_lossy(&result.stdout);
    let lines = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    Ok(lines)
}

/// is_repo returns true or false if the current dir is a git repo
pub fn is_repo() -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Could not get commit hash")
        .stdout;

    let out = String::from_utf8_lossy(&output);
    Ok(out.eq("true"))
}

/// current_branch returns the name of the current branch
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Could not get commit hash");

    let out = String::from_utf8_lossy(&output.stdout);
    Ok(out.trim().to_string())
}

/// is_clean will return a bool based on if the current repo state is clean
pub fn is_clean() -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Could not get commit hash");

    let out = String::from_utf8_lossy(&output.stdout);
    Ok(out.trim().eq(""))
}

/// commit creates a new commit with message
pub fn commit(message: &str, empty: bool) -> Result<()> {
    let mut cmd = Command::new("git");

    cmd.arg("commit");
    cmd.arg("-m");
    cmd.arg(message);

    if empty {
        cmd.arg("--allow-empty");
    }

    let res = cmd.output()?;

    if res.status.success() {
        return Ok(());
    }
    Err(anyhow!("failed to create commit message"))
}

/// Create a temporary WIP commit with all current changes
pub fn create_wip_commit() -> Result<()> {
    // First add all changes
    let add = Command::new("git")
        .args(["add", "."])
        .output()?;

    if !add.status.success() {
        return Err(anyhow!("Failed to stage changes for WIP commit"));
    }

    // Create the WIP commit
    let commit = Command::new("git")
        .args(["commit", "-m", "[SAGE WIP] Temporary commit for sync"])
        .output()?;

    if !commit.status.success() {
        return Err(anyhow!("Failed to create WIP commit"));
    }

    Ok(())
}

/// Pop the most recent WIP commit but keep the changes
pub fn pop_wip_commit() -> Result<()> {
    // Reset the WIP commit but keep changes
    let reset = Command::new("git")
        .args(["reset", "--soft", "HEAD~1"])
        .output()?;

    if !reset.status.success() {
        return Err(anyhow!("Failed to pop WIP commit"));
    }

    Ok(())
}

/// Returns the commit hash of HEAD
pub fn hash() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Could not get commit hash");

    if !output.status.success() {
        return Err(anyhow!("Failed to get commit hash"));
    }
    let hash = String::from_utf8_lossy(&output.stdout);
    Ok(hash.trim().to_string())
}
