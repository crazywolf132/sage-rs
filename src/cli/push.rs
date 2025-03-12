use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct PushArgs {
    /// Force push the branch
    #[clap(short, long)]
    force: bool,
}

impl Run for PushArgs {
    async fn run(&self) -> Result<()> {
        app::push::push(self.force)?;
        Ok(())
    }
}