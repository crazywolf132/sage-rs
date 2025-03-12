use anyhow::Result;
use crate::{errors, git};

pub fn start(name: String) -> Result<()> {

    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // Get the default branch (usually main or master)
    // Uf we can't determine it, default to "main"
    let default_branch = git::repo::default_branch().unwrap_or("main".to_string());

    // Fetching the remote
    git::repo::fetch_remote()?;

    // Pull latest changes for the default branch
    git::repo::pull(&default_branch)?;

    // Create a new branch if it doesn't exist
    git::branch::switch(name, true)?;

    Ok(())
}