use anyhow::Result;
use crate::{ai, errors, git};

#[derive(Default)]
pub struct CommitOptions {
    /// The message to commit with
    pub message: String,
    /// Whether to allow empty commits or not
    pub empty: bool,
    /// Push to remote after committing
    pub push: bool,
    /// Use AI to generate commit message
    pub ai: bool,
}

pub async fn commit(opts: &CommitOptions) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo()? {
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

    // If the user requested that we use AI to generate the commit message, we will do that here.
    let message = if opts.ai {
        println!("✨ AI mode activated. Generating commit message...");
        ai::commit::generate().await?
    } else {
        opts.message.clone()
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
