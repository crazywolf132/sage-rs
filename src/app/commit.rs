use anyhow::Result;
use crate::{ai, errors, git};

#[derive(Default)]
pub struct CommitOptions {
    /// The message to commit with
    pub message: Option<String>,
    /// Whether to allow empty commits or not
    pub empty: bool,
    /// Push to remote after committing
    pub push: bool,
    /// Use AI to generate commit message
    pub ai: bool,
}

pub async fn commit(opts: &CommitOptions) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // Next thing to workout is if there are files staged or not. If there is, we will commit them,
    // if not we will commit all of them.

    let status = git::status::status()?;

    if !status.is_dirty() && !opts.empty {
        return Err(errors::GitError::NoChanges.into());
    }

    if !status.has_staged_changes() {
        // We will stage all changes then.
        git::repo::stage_all()?;
    }

    // Get the commit message - either from AI or user input
    let message = if opts.ai {
        println!("✨ AI mode activated. Generating commit message...");
        ai::commit::generate().await?
    } else {
        // If not using AI, message must be provided
        opts.message.clone().ok_or_else(|| anyhow::anyhow!("Commit message is required when not using AI"))?
    };

    // We will now create the commit.
    git::commit::commit(&message, opts.empty)?;

    if opts.push {
        let current_branch = git::branch::current()?;
        git::branch::push(&current_branch, false)?;
        println!("Pushed changes to remote");
    }

    Ok(())
}
