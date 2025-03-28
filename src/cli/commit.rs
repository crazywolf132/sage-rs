use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Commit {
    /// Commit message
    #[clap(required_unless_present = "ai")]
    message: Option<String>,

    #[clap(short, long)]
    /// Create an empty commit
    empty: bool,

    #[clap(short, long)]
    /// Push changes to remote after committing
    push: bool,

    #[clap(short, long)]
    /// Use ai to generate commit message
    ai: bool,

    #[clap(short = 'y', long = "yes")]
    /// Skip confirmation when using AI-generated commit message
    auto_confirm: bool,
}

impl Run for Commit {
    async fn run(&self) -> Result<()> {
        let mut opts = app::commit::CommitOptions::default();
        opts.empty = self.empty;
        opts.message = self.message.clone();
        opts.push = self.push;
        opts.ai = self.ai;
        opts.auto_confirm = self.auto_confirm;
        
        app::commit::commit(&opts).await?;
        Ok(())
    }
}
