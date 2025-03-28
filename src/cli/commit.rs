use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Commit {
    /// Commit message
    #[clap(help = "The message for your commit. When used with --ai, this message will be ignored and an AI-generated message will be used instead.")]
    message: String,

    #[clap(short, long)]
    /// Create an empty commit
    #[clap(long_help = "Creates a commit even when there are no changes. This is useful for triggering CI/CD pipelines or marking specific points in your repository's history without modifying any files.")]
    empty: bool,

    #[clap(short, long)]
    /// Push changes to remote after committing
    #[clap(long_help = "Automatically pushes your changes to the remote repository after creating the commit. This combines 'git commit' and 'git push' into a single command, saving you an extra step.")]
    push: bool,

    #[clap(short, long)]
    /// Use AI to generate commit message
    #[clap(long_help = "Analyzes your changes and generates a descriptive commit message using AI. The generated message follows the Conventional Commits specification (https://www.conventionalcommits.org/) with appropriate type prefixes like 'feat:', 'fix:', 'docs:', etc. This helps maintain a standardized and meaningful commit history.")]
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
