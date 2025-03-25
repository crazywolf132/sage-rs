use crate::cli::clone;
use crate::cli::commit;
use crate::cli::start;
use crate::cli::status;
use crate::cli::push;
use crate::cli::switch;
use crate::cli::list;
use crate::cli::completion;

use clap::Parser;

use super::hi;

#[derive(Parser, Debug)]
pub enum Cmd {

    /// Start a new feature branch
    Start(start::StartArgs),

    /// Commit changes to the repository
    Commit(commit::Commit),
    
    /// Clone a repository from GitHub
    Clone(clone::CloneArgs),

    /// Show the status of the repository
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

    /// Say hi
    Hi(hi::HiArgs),
}

