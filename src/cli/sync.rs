use anyhow::Result;
use clap::Parser;

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
        crate::app::sync::sync().await
    }
}