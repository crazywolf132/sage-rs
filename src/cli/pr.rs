use anyhow::Result;
use clap::{Parser, Subcommand};

use super::Run;
use crate::app;

/// GitHub Pull Request commands
#[derive(Parser, Debug)]
pub struct PrArgs {
    #[clap(subcommand)]
    pub command: Option<PrCommands>,
}

/// Commands for interacting with GitHub Pull Requests
#[derive(Subcommand, Debug)]
pub enum PrCommands {
    /// Checkout a PR into a local branch
    Checkout(PrCheckoutArgs),
    /// Check the status of a PR
    Status(PrStatusArgs),
    /// Create a new PR
    Create(PrCreateArgs),
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

#[derive(Parser, Debug)]
pub struct PrCreateArgs {
    /// The title for the PR
    #[clap(short, long)]
    pub title: String,

    /// The body for the PR
    #[clap(short, long)]
    pub body: String,

    /// The base branch for the PR
    #[clap(short, long, default_value = "main")]
    pub base_branch: String,

    /// The head branch for the PR
    #[clap(short, long)]
    pub head_branch: Option<String>,

    /// Toggle the PR as draft
    #[clap(long, default_value_t = false)]
    pub draft: bool,
}

impl Run for PrArgs {
    async fn run(&self) -> Result<()> {
        match &self.command {
            Some(PrCommands::Checkout(args)) => pr_checkout(args).await,
            Some(PrCommands::Status(args)) => pr_status(args).await,
            Some(PrCommands::Create(args)) => pr_create(args).await,
            None => pr_status(&PrStatusArgs { pr_number: None }).await,
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

async fn pr_create(args: &PrCreateArgs) -> Result<()> {
    app::pull_create::pull_create(
        args.title.clone(),
        args.body.clone(),
        args.base_branch.clone(),
        args.head_branch.clone(),
        args.draft,
    )
    .await?;
    Ok(())
}
