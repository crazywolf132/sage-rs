mod cmd;
mod templates;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, name = "sage-plugin-dev", about = "Test and debug Sage plugins")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Simulate a pre-push event
    PrePush(cmd::PrePush),

    /// Simulate a post-commit event
    PostCommit(cmd::PostCommit),

    /// Run a plugin's CLI command
    Run(cmd::Run),

    /// Show detailed information about a plugin
    Info(cmd::Info),

    /// Validate a plugin's structure and manifest
    Validate(cmd::Validate),

    /// Initialize a new plugin project (for development)
    Init(cmd::Init),

    /// Simulate a git hook with real repository data
    GitHook(cmd::GitHook),

    /// Trace plugin execution with detailed input/output
    Trace(cmd::Trace),

    /// Benchmark plugin performance
    Benchmark(cmd::Benchmark),

    /// Test plugin with mock data for edge cases
    Mock(cmd::Mock),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::PrePush(c) => c.run()?,
        Cmd::PostCommit(c) => c.run()?,
        Cmd::Run(c) => c.run()?,
        Cmd::Info(c) => c.run()?,
        Cmd::Validate(c) => c.run()?,
        Cmd::Init(c) => c.run()?,
        Cmd::GitHook(c) => c.run()?,
        Cmd::Trace(c) => c.run()?,
        Cmd::Benchmark(c) => c.run()?,
        Cmd::Mock(c) => c.run()?,
    }

    Ok(())
}
