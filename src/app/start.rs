use crate::{errors, git};
use anyhow::Result;

pub fn start(name: &str) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // Get the default branch (usually main or master)
    // If we can't determine it, default to "main"
    let default_branch = git::repo::default_branch().unwrap_or("main".to_string());

    // Fetching the remote
    git::repo::fetch_remote()?;

    // Pull latest changes for the default branch
    git::repo::pull(&default_branch, true)?;

    // Create a new branch if it doesn't exist
    git::branch::switch(name, true)?;
    git::branch::set_upstream(name)?;

    Ok(())
}
