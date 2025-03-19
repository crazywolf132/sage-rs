use anyhow::Result;
use crate::{errors, gh::pulls, git};
use colored::Colorize;

pub async fn pull_checkout(pr_number: u64, branch_name: Option<String>) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // We will get the repo and owner info from the remote URL
    let (owner, repo_name) = git::repo::owner_repo()?;

    let pull_request = pulls::get_pull_request(&owner, &repo_name, pr_number).await?;
    let branch_name = match &branch_name {
        Some(name) => name.clone(),
        None => pull_request.head.ref_field.clone(), // Use the remote branch name
    };

    // Check if the branch already exists locally
    let branch_exists = git::branch::exists(&branch_name);
    if branch_exists {
        // We are going to switch to it and update the branch
        git::branch::switch_new(&branch_name, false)?;

        // Check to see if the branch is dirty.
        let status = git::status::status()?;
        if status.is_clean() {
            // The branch is clean, so we will do a pull.
            git::repo::pull(&branch_name, true)?;
        }

        println!("Switched to branch: {} {}", branch_name.blue(), status.upstream_status());
        return Ok(());
    }

    // Branch doesn't exist, fetch PR and create new branch
    git::repo::fetch(&format!("pull/{}/head:{}", pr_number, branch_name))?;
    git::branch::switch_new(&branch_name, false)?;
    git::branch::set_upstream(&pull_request.head.ref_field)?;

    println!("Switched to branch: {}", branch_name.blue());


    Ok(())
}