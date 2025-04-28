use crate::model::LintFailure;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("lint failed: {0}")]
    Lint(LintFailure),
    #[error("invalid operation: {0}")]
    InvalidOp(String),
    #[error("graph invariant broken: {0}")]
    Graph(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
