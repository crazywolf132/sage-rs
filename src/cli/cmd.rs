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

    /// Commit changes to the repository with optional AI-generated messages and push capability
    #[clap(alias = "c", long_about = "Creates a commit with your changes and optionally pushes to the remote repository.
This command streamlines the git commit workflow by:

1. Verifying you're in a git repository
2. Checking if there are changes to commit
3. Automatically staging all changes if nothing is staged
4. Creating a commit with your message
5. Optionally pushing changes to the remote repository

When used with the --ai flag, it analyzes your changes and generates a descriptive
commit message following the Conventional Commits specification, which helps maintain
a standardized commit history.

The --empty flag allows creating commits with no changes, which can be useful for
triggering CI/CD pipelines or marking specific points in history.

EXAMPLES:
  sage commit \"fix: resolve login issue\"
  sage commit \"update documentation\" --push
  sage commit \"empty commit for CI trigger\" --empty
  sage commit \"initial commit\" --ai")]
    Commit(commit::Commit),
    
    /// Clone a repository from GitHub
    #[clap(long_about = "Clones a GitHub repository using a simplified syntax. This command:

1. Validates the repository name format (owner/repo)
2. Checks if the target directory already exists to prevent overwriting
3. Formats the appropriate GitHub URL based on your protocol preference
4. Clones the repository into a directory named after the repo

The command accepts repositories in the format 'owner/repo' and automatically
constructs the proper GitHub URL, eliminating the need to type the full URL.
You can choose between HTTPS (default) or SSH protocols with the --ssh flag.

EXAMPLES:
  sage clone octocat/Hello-World
  sage clone rust-lang/rust --ssh")]
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
}

