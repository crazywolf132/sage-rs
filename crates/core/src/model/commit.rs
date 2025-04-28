//! Immutable commit snapshot used by core logic.
//! 
//! Contains only the data stack/undo services need. Diff blobs,
//! file lists, etc. stay in the git-adapter layer.

use serde::{Deserialize, Serialize};
use super::{CommitId, CommitMessage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: CommitId,
    pub subject: String,
    pub author: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub body: String,
}

impl Commit {
    pub fn message(&self) -> CommitMessage {
        // In future parse body; for now subject-only.
        CommitMessage::new(&self.subject)
    }
}