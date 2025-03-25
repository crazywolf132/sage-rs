use anyhow::{anyhow, Result};
use crate::{errors, gh::pulls, git};
use colored::Colorize;
use std::process::Command;

pub async fn pull_checkout(pr_number: u64, branch_name: Option<String>) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // We will get the repo and owner info from the remote URL
    let (owner, repo_name) = git::repo::owner_repo()?;

    // Get the PR information from GitHub
    let pull_request = match pulls::get_pull_request(&owner, &repo_name, pr_number).await {
        Ok(pr) => pr,
        Err(e) => {
            // Handle GitHub-specific errors with more helpful messages
            if e.to_string().contains("401") || e.to_string().contains("authentication") {
                eprintln!("{}", "GitHub authentication failed!".red().bold());
                eprintln!("Please set a GitHub token using one of these methods:");
                eprintln!("  1. Set the {} environment variable", "SAGE_GITHUB_TOKEN".yellow());
                eprintln!("  2. Install and authenticate the GitHub CLI with {}", "gh auth login".yellow());
                eprintln!("\nSee {} for more details", "src/gh/README.md".underline());
                return Err(anyhow!("GitHub authentication failed"));
            } else if e.to_string().contains("404") {
                return Err(anyhow!("Pull request #{} not found in {}/{}", pr_number, owner, repo_name));
            }
            return Err(e);
        }
    };

    // Determine branch name (use provided or from PR head reference)
    let branch_name = match &branch_name {
        Some(name) => name.clone(),
        None => pull_request.head.ref_field.clone(), // Use the remote branch name
    };

    // Check if the branch already exists locally
    let branch_exists = git::branch::exists(&branch_name);
    
    if branch_exists {
        // Branch exists - switch to it and update
        println!("Branch {} already exists locally, switching to it...", branch_name.blue());
        
        // Use git command directly for more reliable checkout
        let checkout_result = Command::new("git")
            .arg("checkout")
            .arg(&branch_name)
            .status()?;
            
        if !checkout_result.success() {
            return Err(anyhow!("Failed to checkout existing branch: {}", branch_name));
        }

        // Check if the branch is clean before pulling
        let status = git::status::status()?;
        if status.is_clean() {
            println!("Pulling latest changes from remote...");
            
            // Pull directly with git command
            let pull_result = Command::new("git")
                .arg("pull")
                .arg("--ff-only")
                .status()?;
                
            if !pull_result.success() {
                println!("{}", "Warning: Failed to pull latest changes. Branch may be out of date.".yellow());
            }
        } else {
            println!("{}", "Branch has local changes, not pulling updates.".yellow());
        }

        println!("Switched to branch: {}", branch_name.blue());
    } else {
        // Branch doesn't exist - fetch PR and create branch
        println!("Fetching and checking out pull request #{}...", pr_number);
        
        // Method 1: First try to fetch and checkout directly with one command
        // This is the most reliable way to get all the PR changes
        let checkout_pr_result = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .arg(format!("pull/{}/head:{}", pr_number, branch_name))
            .status()?;
            
        if !checkout_pr_result.success() {
            return Err(anyhow!("Failed to fetch pull request #{}", pr_number));
        }
        
        // Now checkout the branch
        let checkout_result = Command::new("git")
            .arg("checkout")
            .arg(&branch_name)
            .status()?;
            
        if !checkout_result.success() {
            return Err(anyhow!("Failed to checkout branch: {}", branch_name));
        }
        
        // Set the upstream tracking branch
        let upstream_branch = &pull_request.head.ref_field;
        let set_upstream_result = Command::new("git")
            .arg("branch")
            .arg("--set-upstream-to")
            .arg(format!("origin/{}", upstream_branch))
            .status()?;
            
        if !set_upstream_result.success() {
            println!("{}", "Warning: Failed to set upstream branch. Remote tracking not configured.".yellow());
        }

        println!("Successfully checked out pull request #{} to branch: {}", pr_number, branch_name.blue());
        println!("Pull request is from: {}", upstream_branch.yellow());
    }

    Ok(())
}