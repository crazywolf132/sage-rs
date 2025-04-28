//! Pure data types (value objects) - no methods that touch the OS.

pub mod commit;
pub mod commit_id;
pub mod branch;
pub mod commit_message;
pub mod diff_stats;
pub mod git_action;
pub mod undo;
pub mod stack_graph;
pub mod config;

pub use commit::Commit;
pub use commit_id::CommitId;
pub use branch::BranchName;
pub use commit_message::CommitMessage;
pub use diff_stats::DiffStats;
pub use git_action::GitAction;
pub use undo::{UndoOp, UndoLedger};
pub use stack_graph::{StackGraph, MergeResult};
pub use config::StackConfig;

/// Enum returned by linter when messages aren't Conventional.
#[derive(Debug)]
pub enum LintFailure {
    NonConventional,
    EmptySummary,
}

impl std::fmt::Display for LintFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintFailure::NonConventional => write!(f, "commit message is not conventional"),
            LintFailure::EmptySummary => write!(f, "commit message has empty summary"),
        }
    }
}
