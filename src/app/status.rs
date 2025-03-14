use anyhow::Result;
use crate::{errors, git};

pub fn status() -> Result<()> {

    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // // Get the full status
    let status = git::status::status()?;
    println!("{}", status);
    
    Ok(())
}