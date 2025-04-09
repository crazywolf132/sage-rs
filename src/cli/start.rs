use crate::{app, cli::Run, ui::ColorizeExt};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(
    after_help = "This command ensures your new branch starts from the latest version of the default branch,
preventing future merge conflicts and keeping your feature branch up-to-date."
)]
pub struct StartArgs {
    /// The name of the branch to create
    #[clap(
        help = "The name of the branch to create",
        long_help = "The name of the branch to create. This should follow your team's naming convention, such as:
- feature/name for new features
- bugfix/issue-123 for bug fixes
- hotfix/name for urgent fixes"
    )]
    pub name: String,

    /// Optional parent branch to use
    #[clap(
        short,
        long,
        help = "Optional parent branch to use instead of the default branch",
        long_help = "Optional parent branch to use instead of the default branch (main/master).
If specified, the new branch will be created from this branch instead of the default branch."
    )]
    pub parent: Option<String>,
}

impl Run for StartArgs {
    async fn run(&self) -> Result<()> {
        app::start::start(&self.name)?;
        println!("Successfully created branch: {}", self.name.sage());
        Ok(())
    }
}

