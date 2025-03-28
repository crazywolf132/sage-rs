use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

/// Arguments for the list command
///
/// This command doesn't take any arguments but provides a comprehensive
/// view of all branches in the repository with their status information.
#[derive(Parser, Debug)]
#[clap(after_help = "COLOR CODING:
  Green: Current branch
  Cyan: Branch with unpushed commits (ahead of remote)
  Magenta: Branch behind remote (needs pulling)
  Yellow: Diverged branch (both ahead and behind remote)
  Blue: Other branches

SYMBOLS:
  * : Indicates the current branch
  -> : Shows tracking relationship with remote branch
  ↑n : n commits ahead of remote branch
  ↓n : n commits behind remote branch")]
pub struct ListArgs;

impl Run for ListArgs {
    async fn run(&self) -> Result<()> {
        app::list::list()?;
        Ok(())
    }
}