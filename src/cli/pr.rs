use anyhow::Result;
use clap::{Parser, Subcommand};

use super::Run;
use crate::app;

/// GitHub Pull Request commands
#[derive(Parser, Debug)]
pub struct PrArgs {
    #[clap(subcommand)]
    pub command: PrCommands,
}

/// Commands for interacting with GitHub Pull Requests
#[derive(Subcommand, Debug)]
pub enum PrCommands {
    /// Checkout a PR into a local branch
    Checkout(PrCheckoutArgs),
    // Add more PR subcommands here as needed
    Status(PrStatusArgs),
}

#[derive(Parser, Debug)]
pub struct PrCheckoutArgs {
    /// The PR number to checkout
    #[clap(value_parser)]
    pub pr_number: u64,
    
    /// The name of the local branch to create
    #[clap(value_parser)]
    pub branch_name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct PrStatusArgs {
    /// The PR number to checkout
    #[clap(value_parser)]
    pub pr_number: Option<u64>,
}

impl Run for PrArgs {
    async fn run(&self) -> Result<()> {
        match &self.command {
            PrCommands::Checkout(args) => pr_checkout(args).await,
            PrCommands::Status(args) => pr_status(args).await,
        }
    }
}

/// Checkout a PR to a local branch
async fn pr_checkout(args: &PrCheckoutArgs) -> Result<()> {
    app::pull_checkout::pull_checkout(args.pr_number, args.branch_name.clone()).await?;
    Ok(())
}

/// Check the status of a PR
async fn pr_status(args: &PrStatusArgs) -> Result<()> {
    app::pull_status::pull_status(args.pr_number).await?;
    Ok(())
}