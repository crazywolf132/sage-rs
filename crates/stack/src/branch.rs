use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sage_core::model::BranchName;

/// Current life-cycle state of a branch within a stack.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BranchStatus {
    Draft,
    Open,
    Landed,
    Abandoned,
}

/// Everything we know about a branch.
///
/// Feel free to extend - extra fields will serialise automatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: BranchName,
    pub parent: Option<BranchName>,
    pub created: DateTime<Utc>,
    pub hosted: Option<DateTime<Utc>>,
    pub author: String,
    pub status: BranchStatus,
}

impl BranchInfo {
    /// Build info for a *new* branch.
    pub fn new(name: BranchName, parent: Option<BranchName>, author: impl Into<String>) -> Self {
        Self {
            name,
            parent,
            created: Utc::now(),
            hosted: None,
            author: author.into(),
            status: BranchStatus::Draft,
        }
    }
}
