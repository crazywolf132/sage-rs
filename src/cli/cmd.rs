use crate::cli::clean;
use crate::cli::clone;
use crate::cli::commit;
use crate::cli::completion;
use crate::cli::history;
use crate::cli::list;
use crate::cli::pr;
use crate::cli::push;
use crate::cli::start;
use crate::cli::status;
use crate::cli::switch;
use crate::cli::sync;
use crate::cli::nuke;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum Cmd {
    /// Start a new feature branch
    #[clap(
        alias = "s",
        long_about = "Creates and switches to a new feature branch while ensuring you have the latest changes
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
  sage start bugfix/issue-123 --parent release/v2.0"
    )]
    Start(start::StartArgs),

    /// Commit changes to the repository with optional AI-generated messages and push capability
    #[clap(
        alias = "c",
        long_about = "Creates a commit with your changes and optionally pushes to the remote repository.
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
  sage commit \"initial commit\" --ai"
    )]
    Commit(commit::Commit),

    /// Clone a repository from GitHub
    #[clap(
        long_about = "Clones a GitHub repository using a simplified syntax. This command:

1. Validates the repository name format (owner/repo)
2. Checks if the target directory already exists to prevent overwriting
3. Formats the appropriate GitHub URL based on your protocol preference
4. Clones the repository into a directory named after the repo

The command accepts repositories in the format 'owner/repo' and automatically
constructs the proper GitHub URL, eliminating the need to type the full URL.
You can choose between HTTPS (default) or SSH protocols with the --ssh flag.

EXAMPLES:
  sage clone octocat/Hello-World
  sage clone rust-lang/rust --ssh"
    )]
    Clone(clone::CloneArgs),

    /// Show the status of the repository
    #[clap(
        alias = "ss",
        long_about = "Displays a comprehensive view of your repository's current state, showing:

1. Verifies you're in a git repository
2. Shows your current branch name and its relationship to the remote
3. Displays ahead/behind commit counts relative to the upstream branch
4. Indicates if you have stashed changes
5. Lists all staged changes (added, modified, deleted, renamed files)
6. Lists all unstaged changes in your working directory
7. Shows untracked files
8. Provides clear visual indicators for different types of changes

This command gives you a complete picture of your repository's state in a well-formatted,
easy-to-read display that helps you understand exactly what changes exist and where they are
in the git workflow (staged, unstaged, or untracked).

EXAMPLES:
  sage status
  sage s"
    )]
    Status(status::StatusArgs),

    /// Push the current branch to remote
    #[clap(
        alias = "p",
        long_about = "Pushes your current branch to the remote repository with proper tracking setup.
This command streamlines the git push workflow by:

1. Verifying you're in a git repository
2. Identifying your current branch
3. Pushing the branch to the remote repository (origin)
4. Automatically setting up tracking between local and remote branches
5. Providing clear feedback on successful operations

The command handles authentication automatically and ensures proper upstream tracking
is established, which simplifies subsequent pull and push operations.

When used with the --force flag, it performs a force push, which can overwrite remote
history. This should be used with caution, but is useful in specific scenarios like
updating a feature branch after rebasing.

EXAMPLES:
  sage push              # Push current branch to remote
  sage push --force      # Force push current branch to remote
  sage p                 # Using the alias"
    )]
    Push(push::PushArgs),

    /// Switch to a different branch
    #[clap(
        alias = "sw",
        long_about = "Switches to an existing branch with validation checks to prevent common errors.
This command performs several operations to ensure a safe branch switch:

1. Verifies you're in a git repository
2. Checks if the requested branch exists before attempting to switch
3. Prevents switching to the branch you're already on
4. Handles remote branch references (origin/branch-name) automatically
5. Performs a clean checkout to ensure all files are updated

The command accepts both local branch names and remote branch references (e.g., 'origin/feature').
When a remote branch reference is provided, it automatically switches to the corresponding local branch.
If no branch name is provided, it defaults to switching to the 'main' branch.

EXAMPLES:
  sage switch feature-branch
  sage switch origin/feature-branch
  sage sw hotfix/issue-123
  sage switch          # Switches to main branch"
    )]
    Switch(switch::SwitchArgs),

    /// List all branches in the repository with status information
    #[clap(
        alias = "l",
        long_about = "Displays a comprehensive list of all local branches in the repository with detailed status information:

1. Verifies you're in a git repository
2. Lists all local branches sorted by most recent commit date
3. Clearly marks the current branch with an asterisk (*)
4. Shows tracking relationships between local and remote branches
5. Displays ahead/behind commit counts with intuitive arrows (↑ for ahead, ↓ for behind)
6. Uses color coding to indicate branch status:
   - Current branch: green
   - Branches with unpushed commits: cyan
   - Branches behind remote: magenta
   - Diverged branches (both ahead and behind): yellow
   - Other branches: blue

This command provides a quick overview of all your branches and their synchronization status
with remote branches, helping you understand which branches need attention (pushing, pulling,
or resolving divergence).

EXAMPLES:
  sage list
  sage l"
    )]
    List(list::ListArgs),

    /// Generate shell completions for Bash, Zsh, or Fish
    #[clap(
        long_about = "Generates shell completion scripts that enable tab-completion for sage commands and arguments.
This command outputs completion scripts to stdout, which you can redirect to the appropriate location for your shell:

1. Generates a completion script for the specified shell (Bash, Zsh, or Fish)
2. Outputs the script to stdout, which you can redirect to a file
3. Once installed, provides intelligent tab completion for all sage commands, subcommands, and options

Shell completions significantly improve your CLI experience by:
- Reducing typing and preventing typos
- Showing available commands and options as you type
- Displaying parameter hints and possible values

INSTALLATION INSTRUCTIONS:

Bash:
  sage completion bash > ~/.bash_completion.d/sage
  # Add to ~/.bashrc if not already sourcing completion directory:
  # source ~/.bash_completion.d/sage

Zsh:
  # Create directory if it doesn't exist
  mkdir -p ~/.zsh/completions
  sage completion zsh > ~/.zsh/completions/_sage
  # Add to ~/.zshrc if not already in fpath:
  # fpath=(~/.zsh/completions $fpath)
  # autoload -U compinit && compinit

Fish:
  sage completion fish > ~/.config/fish/completions/sage.fish

EXAMPLES:
  sage completion bash
  sage completion zsh
  sage completion fish"
    )]
    Completion(completion::CompletionArgs),

    /// GitHub Pull Request commands
    #[clap(
        long_about = "Provides commands for interacting with GitHub Pull Requests, allowing you to:

1. Checkout pull requests locally for review and testing
2. View detailed information about pull requests including status, description, and CI checks
3. Seamlessly integrate GitHub workflows into your local development process

The PR commands automatically handle authentication with GitHub using either a SAGE_GITHUB_TOKEN
environment variable or the GitHub CLI if installed. They provide a streamlined interface for
common PR operations that would otherwise require multiple manual steps.

EXAMPLES:
  sage pr checkout 123                  # Checkout PR #123 to a local branch
  sage pr checkout 123 feature/test     # Checkout PR #123 to a specific branch name
  sage pr status                        # Show status of PR associated with current branch
  sage pr status 456                    # Show status of PR #456"
    )]
    Pr(pr::PrArgs),

    /// Synchronize the repository with the remote
    #[clap(
        long_about = "Synchronizes your current branch with the default branch (main/master) while preserving your changes.
This command performs several git operations automatically:

1. Verifies you're in a git repository
2. Fetches the latest changes from the remote repository
3. Temporarily stashes any uncommitted changes
4. Switches to the default branch (usually main or master)
5. Updates the default branch with latest changes (git pull)
6. Switches back to your original branch
7. Intelligently updates your branch with changes from the default branch:
   - If your branch has diverged (both ahead and behind), it rebases your changes
   - Otherwise, it performs a standard merge
8. Restores any stashed changes
9. Pushes your updated branch to the remote

This workflow ensures your branch stays up-to-date with the latest changes from the default branch,
reducing the likelihood of complex merge conflicts later. It's particularly useful for long-lived
feature branches that need to incorporate ongoing changes from the main codebase.

EXAMPLES:
  sage sync"
    )]
    Sync(sync::SyncArgs),

    /// Nuke the working directory, discarding all changes and untracked files
    #[clap(
        long_about = "Forces a hard reset of the working directory and removes untracked files.",
        alias = "n"
    )]
    Nuke(nuke::NukeArgs),

    /// Cleans up all dead branches
    Clean(clean::CleanArgs),

    /// History of commits
    #[clap(alias = "h")]
    History(history::History),
}
