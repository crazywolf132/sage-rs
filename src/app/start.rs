use anyhow::Result;
use crate::{errors, git};

pub fn start(name: String) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    let status = git::status::status()?;

    // Check if there are unstaged changes and stash them if needed
    let stashed = if status.is_dirty() {
        println!("Detected unstaged changes. Temporarily stashing them...");
        git::stash::stash_changes()?;
        true
    } else {
        false
    };

    // Get the default branch (usually main or master)
    // If we can't determine it, default to "main"
    let default_branch = git::repo::default_branch().unwrap_or("main".to_string());

    // Fetching the remote
    git::repo::fetch_remote()?;

    // Pull latest changes for the default branch
    git::repo::pull(&default_branch, true)?;

    // Create a new branch if it doesn't exist
    git::branch::switch(name, true)?;

    // Restore stashed changes if we stashed them earlier
    if stashed {
        println!("Restoring your changes on the new branch...");
        git::stash::apply_stash()?;
    }

    Ok(())
}