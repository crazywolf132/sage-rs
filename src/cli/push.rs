use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct PushArgs {
    /// Force push the branch
    #[clap(short, long, long_help = "Force push the current branch to the remote repository,
    overwriting the remote branch history. This is useful when you've rebased your branch
    or amended commits and need to update the remote. Use with caution as it can overwrite
    changes others may have pushed.")]
    force: bool,
}

impl Run for PushArgs {
    async fn run(&self) -> Result<()> {
        app::push::push(self.force)?;
        Ok(())
    }
}