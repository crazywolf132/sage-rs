use thiserror::Error;

use crate::branch::BranchId;

/// Errors that can arise when manipulating stacks/graphs.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StackError {
    #[error("stack \"{0}\" already exists")]
    StackExists(String),

    #[error("branch \"{0}\" already exists")]
    BranchExists(BranchId),

    #[error("unknown branch \"{0}\"")]
    UnknownBranch(BranchId),

    #[error("cannot reorder children of \"{parent}\": lists differ\nexpected: {expected:?}\n   found: {got:?}")]
    InvalidReorder {
        parent: String,
        expected: Vec<BranchId>,
        got: Vec<BranchId>,
    },

    #[error("I/O – {0}")]
    Io(#[from] std::io::Error),

    #[error("serde – {0}")]
    Serde(#[from] serde_json::Error),
}
