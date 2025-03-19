use anyhow::Result;
use crate::{errors, git};

pub fn commit(message: String) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // Next thing to workout is if there are files staged or not. If there is, we will commit them,
    // if not we will commit all of them.

    let status = git::status::status()?;
    println!("{}", status);

    if !status.has_staged_changes() {
        // We will stage all changes then.
        git::repo::stage_all()?;
    }

    // We will now create the commit.
    let _ = git::commit::commit(message);

    Ok(())
}
