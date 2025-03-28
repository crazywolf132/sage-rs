use anyhow::Result;
use crate::{errors, git};
use colored::Colorize;

pub fn push(force: bool) -> Result<()> {

    // Check to ensure we are in a repo first.
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // Getting the current branch name
    let current_branch = git::branch::current()?;

    // Pushing the branch to remote
    git::branch::push(&current_branch, force)?;

    println!("Successfully pushed branch: {}", current_branch.blue());

    Ok(())
}