use anyhow::Result;
use clap::Parser;
use crate::{app, git};

/// Arguments for the sync command
#[derive(Parser, Debug)]
#[clap(after_help = "The sync command is designed to keep your feature branches up-to-date with the default branch.
It handles the complex workflow of stashing changes, updating branches, and applying the right
strategy (merge or rebase) based on your branch's relationship with the default branch.

This command is particularly useful in these scenarios:
- When working on a long-lived feature branch that needs to incorporate ongoing changes
- Before creating a pull request to ensure your branch has the latest changes
- After pulling in changes from other team members to keep everything synchronized

The command automatically detects if your branch has diverged from the default branch
(both ahead and behind) and uses rebase in that case to maintain a cleaner history.")]
pub struct SyncArgs;

impl SyncArgs {
    pub async fn run(&self) -> Result<()> {
        match app::sync::sync() {
            Ok(_) => Ok(()),
            Err(_) => {
                // if there was an error doing this, we will try and give the user their changes back
                // so as not to break their work.
                if git::stash::has_stash()? {
                    git::stash::apply_stash()?;
                }
                Ok(())
            }
        }
    }
}