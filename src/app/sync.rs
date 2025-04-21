use crate::{errors, git};
use anyhow::{anyhow, Result};
use crate::ui::ColorizeExt;

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
    // Determine the primary remote and remote default branch ref
    let primary_remote = git::repo::primary_remote()?;
    let remote_default = format!("{}/{}", primary_remote, default_branch);

    // Determine if there are any local changes to stash
    let status = git::status::status()?;
    let mut stashed = false;
    if status.has_changes() || status.has_staged_changes() {
        println!("Stashing local changes...");
        git::stash::stash_changes()?;
        stashed = true;
    }

    // Fetch latest changes from remote to get an up-to-date picture
    println!("Fetching remote changes...");
    if let Err(err) = git::repo::fetch_remote() {
        println!("⚠️  Warning: Failed to fetch remote changes: {}. Continuing...", err);
    }

    // If we're on the default branch, just pull and we're done
    if current_branch == default_branch {
        println!("On default branch, pulling latest changes...");
        git::repo::pull(&default_branch, true)?;
        // Restore stashed changes if any
        if stashed {
            println!("Restoring local changes...");
            git::stash::apply_stash()?;
        }
        println!("✨ Successfully updated default branch!");
        return Ok(());
    }

    // We're on a feature branch - let's be smart about how we sync
    println!("Analyzing branch state...");

    // First update the default branch tracking ref without switching to it
    if let Err(err) = git::repo::fetch_branch(&default_branch) {
        println!("⚠️  Warning: Failed to fetch branch {}: {}. Continuing...", default_branch.sage(), err);
    }

    // Determine the best sync strategy based on branch state relative to remote default branch
    let (ahead_count, behind_count) = git::repo::ahead_behind(&current_branch, &remote_default)?;
    let diverged = ahead_count > 0 && behind_count > 0;
    let behind = behind_count > 0;
    let ahead = ahead_count > 0;

    if diverged {
        // Branch has diverged - try to rebase onto remote default, then fallback to merge
        println!("Branch has diverged from {}...", default_branch.sage());
        match git::branch::rebase(&remote_default) {
            Ok(_) => {
                println!("Rebase successful, pushing changes...");
                if let Err(err) = git::branch::push(&current_branch, false) {
                    println!("⚠️  Warning: Failed to push branch {}: {}", current_branch, err);
                }
            }
            Err(_) => {
                println!("Rebase encountered conflicts, falling back to merge...");
                git::branch::abort_rebase()?;
                match git::branch::merge(&remote_default) {
                    Ok(_) => {
                        println!("Merge successful, pushing changes...");
                        if let Err(err) = git::branch::push(&current_branch, false) {
                            println!("⚠️  Warning: Failed to push branch {}: {}", current_branch, err);
                        }
                    }
                    Err(_) => {
                        // Both rebase and merge failed - need manual intervention
                        println!("\n⚠️  Could not automatically sync branch:");
                        println!("1. Your branch has diverged significantly from {}", default_branch.sage());
                        println!("2. Both rebase and merge resulted in conflicts");
                        println!("\nRecommended actions:");
                        println!("1. Manually merge {} into your branch", default_branch.sage());
                        println!("2. Resolve the conflicts");
                        println!("3. Run sage sync again");
                        return Err(anyhow!("Could not automatically sync diverged branch"));
                    }
                }
            }
        }
    } else if behind {
        // Branch is behind remote default - rebase onto it, then push
        println!("Branch is behind {}, updating...", default_branch.sage());
        // Try to rebase; on failure, attempt merge
        if let Err(err) = git::branch::rebase(&remote_default) {
            println!("⚠️  Warning: Rebase failed ({}); attempting merge...", err);
            if let Err(err2) = git::branch::merge(&remote_default) {
                println!("⚠️  Warning: Merge also failed ({}). You may need to resolve conflicts manually.", err2);
            }
        }
        // Push changes
        println!("Updating complete, pushing changes...");
        if let Err(err) = git::branch::push(&current_branch, false) {
            println!("⚠️  Warning: Failed to push branch {}: {}", current_branch, err);
        }
    } else if ahead {
        // Branch has unique commits - push to remote
        println!("Pushing commits to remote...");
        if let Err(err) = git::branch::push(&current_branch, false) {
            println!("⚠️  Warning: Failed to push branch {}: {}", current_branch, err);
        }
    }

    // Restore stashed changes if any
    if stashed {
        println!("Restoring local changes...");
        git::stash::apply_stash()?;
    }

    println!("✨ Successfully synced branch {}!", current_branch.sage());

    Ok(())
}
