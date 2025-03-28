use crate::cli::clone;
use crate::cli::commit;
use crate::cli::start;
use crate::cli::status;
use crate::cli::push;
use crate::cli::switch;
use crate::cli::list;
use crate::cli::completion;
use crate::cli::pr;
use crate::cli::sync;
use crate::cli::clean;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum Cmd {

    /// Start a new feature branch
    #[clap(long_about = "Creates and switches to a new feature branch while ensuring you have the latest changes
from the remote repository. This command performs several git operations automatically:

1. Verifies you're in a git repository
2. Temporarily stashes any uncommitted changes
3. Determines the default branch (usually main or master)
4. Fetches the latest changes from the remote (git fetch --all --prune)
5. Updates the default branch with latest changes (git pull origin <default-branch> --ff-only)
6. Creates and switches to the new branch (git switch -c <branch-name>)
7. Restores any stashed changes to your new branch

This workflow ensures your new branch starts from the latest version of the default branch,
preventing future merge conflicts and keeping your feature branch up-to-date.

EXAMPLES:
  sage start new-feature
  sage start bugfix/issue-123 --parent release/v2.0")]
    Start(start::StartArgs),

    /// Commit changes to the repository
    #[clap(alias = "c")]
    Commit(commit::Commit),
    
    /// Clone a repository from GitHub
    Clone(clone::CloneArgs),

    /// Show the status of the repository
    #[clap(alias = "s")]
    Status(status::StatusArgs),

    /// Push the current branch to remote
    #[clap(alias = "p")]
    Push(push::PushArgs),

    /// Switch to a different branch
    #[clap(alias = "sw")]
    Switch(switch::SwitchArgs),

    /// List all branches
    #[clap(alias = "l")]
    List(list::ListArgs),
    
    /// Generate shell completions
    Completion(completion::CompletionArgs),

    /// GitHub Pull Request commands
    Pr(pr::PrArgs),

    /// Synchronize the repository with the remote
    Sync(sync::SyncArgs),

    /// Cleans up all dead branches
    Clean(clean::CleanArgs),
}

