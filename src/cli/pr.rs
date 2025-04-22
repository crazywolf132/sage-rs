use anyhow::Result;
use clap::{Parser, Subcommand};

use super::Run;
use crate::app;

/// GitHub Pull Request commands
#[derive(Parser, Debug)]
#[clap(after_help = "Authentication is handled automatically using either:
1. The SAGE_GITHUB_TOKEN environment variable
2. The GitHub CLI (gh) if installed and authenticated

See src/gh/README.md for more details on authentication.")]
pub struct PrArgs {
    #[clap(subcommand)]
    pub command: Option<PrCommands>,
}

/// Commands for interacting with GitHub Pull Requests
#[derive(Subcommand, Debug)]
pub enum PrCommands {
    /// Checkout a PR into a local branch
    #[clap(long_about = "Fetches a GitHub pull request and checks it out to a local branch for review and testing.
This command performs several operations automatically:

1. Verifies you're in a git repository
2. Retrieves pull request information from GitHub
3. Creates a local branch with the PR's changes
4. Sets up proper tracking with the remote branch

If the branch already exists locally, it will switch to it and update it with the latest changes
from the remote if the branch is clean.

EXAMPLES:
  sage pr checkout 123                  # Checkout PR #123 using the PR's branch name
  sage pr checkout 123 feature/test     # Checkout PR #123 to a branch named 'feature/test'")]
    Checkout(PrCheckoutArgs),
    
    /// Show status and details of a pull request
    #[clap(long_about = "Displays detailed information about a GitHub pull request, including:

1. PR title, description, and URL
2. Current status (open, closed, merged)
3. Source and target branches
4. CI check status with visual indicators
5. Recent commits with authors

If no PR number is provided, it attempts to find a PR associated with the current branch.
This is useful for quickly checking the status of the PR you're currently working on.

EXAMPLES:
  sage pr status         # Show status of PR associated with current branch
  sage pr status 456     # Show status of PR #456")]

    Status(PrStatusArgs),
    /// Create a new PR
    #[clap(alias = "c", long_about = "Creates a new pull request with optional AI-generated title and body.")]
    Create(PrCreateArgs),
}

#[derive(Parser, Debug)]
pub struct PrCheckoutArgs {
    /// The PR number to checkout
    #[clap(value_parser, long_help = "The pull request number to checkout. This must be a valid PR number for the current repository.")]
    pub pr_number: u64,

    /// The name of the local branch to create
    #[clap(value_parser, long_help = "Optional custom name for the local branch to create. If not provided, the PR's original branch name will be used.")]
    pub branch_name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct PrStatusArgs {
    /// The PR number to check status for
    #[clap(value_parser, long_help = "Optional PR number to check status for. If not provided, attempts to find a PR associated with the current branch.")]
    pub pr_number: Option<u64>,
}

#[derive(Parser, Debug)]
pub struct PrCreateArgs {
    /// The title for the PR
    #[clap(short, long)]
    pub title: Option<String>,

    /// The body for the PR
    #[clap(short, long)]
    pub body: Option<String>,

    /// The base branch for the PR
    #[clap(short, long)]
    pub base_branch: Option<String>,

    /// The head branch for the PR
    #[clap(short, long)]
    pub head_branch: Option<String>,

    /// Toggle the PR as draft
    #[clap(long)]
    pub draft: Option<bool>,
    
    /// Use AI to generate title and body
    #[clap(short = 'a', long, default_value = "false")]
    pub ai: bool,
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
///
/// This function fetches a GitHub pull request and checks it out to a local branch.
/// It handles authentication, fetching PR data, and creating/updating the local branch.
/// If the branch already exists, it will switch to it and update it with the latest changes.
async fn pr_checkout(args: &PrCheckoutArgs) -> Result<()> {
    app::pull_checkout::pull_checkout(args.pr_number, args.branch_name.clone()).await?;
    Ok(())
}

/// Check the status of a PR
///
/// This function displays detailed information about a GitHub pull request,
/// including its status, description, CI checks, and recent commits.
/// If no PR number is provided, it attempts to find a PR associated with the current branch.
async fn pr_status(args: &PrStatusArgs) -> Result<()> {
    app::pull_status::pull_status(args.pr_number).await?;
    Ok(())
}

async fn pr_create(args: &PrCreateArgs) -> Result<()> {
    // Use interactive mode if any required fields are missing and AI is not enabled
    let interactive = (args.title.is_none() || args.body.is_none()) && !args.ai;
    
    app::pull_create::pull_create(
        args.title.clone(),
        args.body.clone(),
        args.base_branch.clone(),
        args.head_branch.clone(),
        args.draft.unwrap_or(false),
        interactive,
        args.ai,
    )
    .await?;
    Ok(())
}
