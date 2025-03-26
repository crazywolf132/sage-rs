use crate::{app, cli::Run};
use clap::Parser;
use anyhow::Result;
use crate::cli::completion::value_completion;

#[derive(Parser, Debug)]
#[clap(after_help = "NOTES:
- This command is safer than using 'git checkout' or 'git switch' directly as it performs validation checks.
- When switching branches, any uncommitted changes will remain in your working directory.
- If you need to create a new branch, use the 'sage start' command instead.
- To list available branches, use 'sage list' or 'sage l'.

RELATED COMMANDS:
  sage start   - Create and switch to a new branch
  sage list    - List all branches
  sage status  - Show the status of the repository")]
pub struct SwitchArgs {
    /// The name of the branch to switch to
    #[clap(
        value_parser = value_completion::branch_names,
        long_help = "The name of the branch you want to switch to. This can be:
- A local branch name (e.g., 'feature-branch')
- A remote branch reference (e.g., 'origin/feature-branch')
- Omitted to switch to the default 'main' branch

The command will validate that the branch exists before attempting to switch.
Branch name completion is provided to help you select from existing branches."
    )]
    pub name: Option<String>,
}

impl Run for SwitchArgs {
    async fn run(&self) -> Result<()> {
        app::switch::switch(self.name.clone().unwrap_or("main".to_string()))?;
        Ok(())
    }
}