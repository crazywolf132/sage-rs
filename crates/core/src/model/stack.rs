use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{branch::Branch, BranchName};

/// Represents a complete stack
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stack {
    pub name: BranchName,      // e.g. "feature-stack"
    base_branch: String,       // e.g. "main"
    pub head_branch: Branch,   // The first branch in the stack
    pub branches: Vec<Branch>, // Ordered list of branches in th stack
    created_at: DateTime<Utc>, // Timestamp for creation
    updated_at: DateTime<Utc>, // Last update timestamp
}
