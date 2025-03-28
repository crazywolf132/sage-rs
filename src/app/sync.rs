use crate::{errors, git};
use anyhow::{anyhow, Result};

pub fn sync() -> Result<()> {
    // First we need to check to see if we are in a repo
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // Get initial status to show what's changing
    let initial_status = git::status::status()?;
    println!("\nInitial state:");
    println!("{}\n", initial_status);

    // We will now fetch the remote.
    git::repo::fetch_remote()?;

    // Get the current status to check if we've diverged
    let status = git::status::status()?;

    // We will now stash the users changes.
    git::stash::stash_changes()?;

    let current_branch = git::branch::current()?;

    // We will get the default branch name,
    let default_branch = git::repo::default_branch()?;

    // We will now switch to the default branch. Only if it is not the current branch.
    if current_branch != default_branch {
        git::branch::switch(&default_branch, false)?;
    }

    // We will now pull the latest changes.
    git::repo::pull(&default_branch, true)?;

    // If the default branch is the same as the current branch, we will exist early.
    if current_branch == default_branch {
        // Get final status
        let final_status = git::status::status()?;
        println!("\nFinal state:");
        println!("{}\n", final_status);
        println!("✨ Successfully synced repository on branch '{}'!", current_branch);
        return Ok(());
    }

    // We will now switch back to the previous branch.
    git::branch::switch(&current_branch, false)?;

    // If the branch has diverged, we'll rebase instead of merge
    if status.is_diverged() {
        git::branch::rebase(&default_branch)?;
    } else {
        // Otherwise, we'll merge the default branch into the current branch
        git::branch::merge(&default_branch)?;
    }

    // We will now apply the stash. -- If there is actually a stash.
    if git::stash::has_stash()? {
        git::stash::apply_stash()?;
    }

    let conflicting_files = git::branch::conflicting_files()?;
    if !conflicting_files.is_empty() {
        println!("The following files are conflicting:");
        for file in conflicting_files {
            println!("  {}", file);
        }
        return Err(anyhow!("There are conflicting files."));
    }

    // We will now push the changes to the remote.
    git::branch::push(&default_branch, false)?;

    // Get final status
    let final_status = git::status::status()?;
    println!("\nFinal state:");
    println!("{}\n", final_status);
    println!("✨ Successfully synced repository on branch '{}'!", current_branch);

    Ok(())
}
