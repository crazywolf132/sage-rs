use anyhow::{anyhow, Result};
use crate::{git, errors};
use crate::tui;

// Removed direct Command use: using git::repo methods

pub async fn nuke() -> Result<()> {
    // Ensure we're in a git repository
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    // Gather current status
    let status = git::status::status()?;

    // Show which files will be discarded or removed
    if status.has_changes() || !status.untracked.is_empty() || !status.ignored.is_empty() {
        println!("\nThe following changes will be discarded:");
        for file in status.all_modified_files() {
            println!("  Modified: {}", file);
        }
        for file in status.all_added_files() {
            println!("  Added: {}", file);
        }
        for file in status.all_deleted_files() {
            println!("  Deleted: {}", file);
        }
        for (from, to) in status.all_renamed_files() {
            println!("  Renamed: {} -> {}", from, to);
        }
        for (from, to) in status.all_copied_files() {
            println!("  Copied: {} -> {}", from, to);
        }
        if !status.untracked.is_empty() {
            println!("\nThe following untracked files/directories will be removed:");
            for file in &status.untracked {
                println!("  {}", file);
            }
        }
        if !status.ignored.is_empty() {
            println!("\nThe following ignored files/directories will be removed:");
            for file in &status.ignored {
                println!("  {}", file);
            }
        }
    } else {
        println!("Working directory is clean, nothing to discard.");
    }

    // Confirm with user
    tui::confirm("Are you sure you want to proceed?")?;

    // Proceed with reset and clean
    git::repo::reset_hard_head()?;
    git::repo::clean_untracked()?;

    println!("✨ Nuked working directory: Discarded all changes and untracked files.");
    Ok(())
}
