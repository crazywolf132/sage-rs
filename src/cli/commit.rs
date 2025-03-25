use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Commit {
    /// Commit message
    message: String,

    #[clap(short, long)]
    /// Create an empty commit
    empty: bool,

    #[clap(short, long)]
    /// Push changes to remote after committing
    push: bool,

    #[clap(short, long)]
    /// Use ai to generate commit message
    ai: bool,
}

impl Run for Commit {
    async fn run(&self) -> Result<()> {
        println!("Committing with message: {}", self.message);

        let mut opts = app::commit::CommitOptions::default();
        opts.empty = self.empty;
        opts.message = self.message.to_string();
        opts.push = self.push;
        opts.ai = self.ai;
        
        app::commit::commit(&opts).await?;
        Ok(())
    }
}
