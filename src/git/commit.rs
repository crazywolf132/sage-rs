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
pub fn commit(message: String) -> Result<()> {
    let res = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()?;

    if res.status.success() {
        return Ok(());
    }
    Err(anyhow!("failed to create commit message"))
}
