use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
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
    let branches: Vec<String> = output
        .split('\n')
        .map(|s| s.replace("origin/", "").replace("origin", "").to_string())
        .filter(|b| !b.is_empty())
        .collect();
    Ok(branches)
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub date: String,
    pub author: String,
}

pub fn log(branch: &str, limit: usize, stats: bool, all: bool) -> Result<Vec<String>> {
    let mut cmd = Command::new("git");
    cmd.arg("log");
    cmd.arg("--pretty=format:%H%x00%an%x00%at%x00%s");

    if limit > 0 && !all {
        cmd.arg(format!("-n {}", limit));
    }

    if stats {
        cmd.arg("--numstat");
    }

    if !branch.is_empty() {
        cmd.arg(branch);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list commits"));
    }

    let output = String::from_utf8(output.stdout)?;
    let commits: Vec<String> = output.split('\n').map(|s| s.to_string()).collect();
    Ok(commits)
}

/// lists all commits on the current branch
pub fn commits() -> Result<Vec<Commit>> {
    let log_result = log("", 0, false, true)?;
    let mut commits = Vec::new();

    for log_line in log_result {
        let parts: Vec<&str> = log_line.split('\x00').collect();
        if parts.len() < 4 {
            continue; // We want to avoid these lines, as they are not proper commits.
        }

        let hash = parts[0].to_string();
        let author = parts[1];
        let timestamp = parts[2];
        let message = parts[3];

        // Format the date from Unix timestamp
        let formatted_date = if let Ok(ts) = timestamp.parse::<i64>() {
            // Use the non-deprecated approach
            if let Some(dt) = DateTime::<Utc>::from_timestamp(ts, 0) {
                format!("{}", dt.format("%a %b %d %Y"))
            } else {
                "Unknown date".to_string()
            }
        } else {
            "Unknown date".to_string()
        };

        // Get short hash (first 7 characters)
        let short_hash = if hash.len() >= 7 {
            hash[..7].to_string()
        } else {
            hash.clone()
        };

        commits.push(Commit {
            hash: short_hash,
            author: author.to_string(),
            date: formatted_date,
            message: message.to_string(),
        });
    }

    Ok(commits)
}
