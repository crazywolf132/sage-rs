use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Commit {
    /// Commit message
    message: String,
}

impl Run for Commit {
    async fn run(&self) -> Result<()> {
        println!("Committing with message: {}", self.message);
        app::commit::commit(self.message.to_string())?;
        Ok(())
    }
}
