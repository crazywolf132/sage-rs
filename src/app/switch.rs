use anyhow::{anyhow, Result};
use crate::{errors, git, tui};
use colored::Colorize;

pub fn switch(name: Option<String>) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // If no branch name is provided, show the TUI selector
    let branch_name = match name {
        Some(name) => name,
        None => tui::branch::select_branch()?,
    };

    let mut duplicate_branch_requested_name = branch_name.clone(); 
    if duplicate_branch_requested_name.starts_with("origin/") {
        duplicate_branch_requested_name = duplicate_branch_requested_name.replacen("origin/", "", 1);
    }

    // We are here, so obviously we are within a repo.
    // Getting the current branch name
    let current_branch = git::branch::current()?;

    // Check if the branch the user requested is the same.
    if duplicate_branch_requested_name == current_branch {
        return Err(anyhow!("Cannot switch to the same branch"));
    }

    // For safety, and to provide a better user experience, we will check if the branch exists.
    if !git::branch::exists(duplicate_branch_requested_name.as_str()) {
        return Err(anyhow!("Branch {} does not exist", duplicate_branch_requested_name.blue()));
    }

    // We will now try and checkout the branch
    git::branch::switch_new(&branch_name, false)?;

    println!("Now on branch: {}", duplicate_branch_requested_name.blue());

    Ok(())
}