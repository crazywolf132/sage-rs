use anyhow::{anyhow, Result};
use std::process::Command;

use super::repo::default_branch;

/// local returns a list of local branches
pub fn local() -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--list")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!("Failed to list local branches"));
    }
    let output = String::from_utf8(result.stdout)?;
    let branches: Vec<&str> = output.split('\n').filter(|b| !b.is_empty()).collect();
    Ok(branches.iter().map(|b| b.to_string()).collect())
}

/// merged returns a list of merged branches
pub fn merged() -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--merged")
        .arg(default_branch()?)
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!("Failed to list merged branches"));
    }

    let output = String::from_utf8(result.stdout)?;
    let branches: Vec<&str> = output.split('\n').filter(|b| !b.is_empty()).collect();
    Ok(branches.iter().map(|b| b.to_string()).collect())
}

/// remote returns a list of remote branches
pub fn remote() -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--list")
        .arg("--remotes")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!("Failed to list remote branches"));
    }

    let output = String::from_utf8(result.stdout)?;
    let branches: Vec<String> = output.split('\n').map(|s| s.replace("origin/", "").replace("origin", "").to_string()).filter(|b| !b.is_empty()).collect();
    Ok(branches)
}