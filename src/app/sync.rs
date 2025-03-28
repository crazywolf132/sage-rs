use crate::{errors, git};
use anyhow::{anyhow, Result};

/// Sync the current branch with its upstream/parent branch
/// 
/// This is a smart sync that:
/// 1. Detects the best sync strategy based on branch state
/// 2. Tries to minimize conflicts by analyzing changes
/// 3. Handles everything automatically without user intervention
/// 4. Recovers gracefully from errors when possible
pub fn sync() -> Result<()> {
    // Check if we're in a repo
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // Get current branch and default branch
    let current_branch = git::branch::current()?;
    let default_branch = git::repo::default_branch()?;

    // Get initial status
    let status = git::status::status()?;
    println!("\nInitial state:");
    println!("{}\n", status);

    // Fetch latest changes from remote to get an up-to-date picture
    println!("Fetching remote changes...");
    git::repo::fetch_remote()?;

    // If we're on the default branch, just pull and we're done
    if current_branch == default_branch {
        println!("On default branch, pulling latest changes...");
        git::repo::pull(&default_branch, true)?;
        println!("✨ Successfully updated default branch!");
        return Ok(());
    }

    // We're on a feature branch - let's be smart about how we sync
    println!("Analyzing branch state...");

    // First update the default branch without switching to it
    // This gives us the latest state to work with
    git::repo::fetch_branch(&default_branch)?;

    // Check if there are any local changes that aren't pushed
    let has_local_changes = status.has_changes() || status.has_staged_changes();

    // If we have local changes, commit them as a WIP
    if has_local_changes {
        println!("Creating temporary commit for local changes...");
        git::commit::create_wip_commit()?;
    }

    // Determine the best sync strategy based on branch state
    let diverged = status.behind_count > 0 && status.ahead_count > 0;
    let behind = status.behind_count > 0;
    let ahead = status.ahead_count > 0;

    if diverged {
        // Branch has diverged - try to rebase but fall back to merge if needed
        println!("Branch has diverged from {}...", default_branch);
        
        // Try rebase first
        if let Err(_) = git::branch::rebase(&default_branch) {
            println!("Rebase encountered conflicts, falling back to merge...");
            // Abort the failed rebase
            git::branch::abort_rebase()?;
            
            // Try merge instead
            if let Err(_) = git::branch::merge(&default_branch) {
                // Both rebase and merge failed - need manual intervention
                println!("\n⚠️  Could not automatically sync branch:");
                println!("1. Your branch has diverged significantly from {}", default_branch);
                println!("2. Both rebase and merge resulted in conflicts");
                println!("\nRecommended actions:");
                println!("1. Manually merge {} into your branch", default_branch);
                println!("2. Resolve the conflicts");
                println!("3. Run sage sync again");
                return Err(anyhow!("Could not automatically sync diverged branch"));
            }
        }
    } else if behind {
        // We're just behind - do a rebase
        println!("Branch is behind {}, updating...", default_branch);
        git::branch::rebase(&default_branch)?;
    } else if ahead && !has_local_changes {
        // We're ahead with clean commits - try to push
        println!("Pushing commits to remote...");
        git::branch::push(&current_branch, false)?;
    }

    // If we created a WIP commit, handle it now
    if has_local_changes {
        // Pop the WIP commit but keep the changes
        println!("Restoring uncommitted changes...");
        git::commit::pop_wip_commit()?;
    }

    // Get final status
    let final_status = git::status::status()?;
    println!("\nFinal state:");
    println!("{}\n", final_status);
    println!("✨ Successfully synced branch '{}'!", current_branch);

    Ok(())
}
