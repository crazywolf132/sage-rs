//! `CommitId` - strongly-typed Git object ID (SHA-1 or SHA-256).

use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref, str::FromStr};

/// Accept 7-40 hex (SHA-1) or 64 hex (SHA-256).
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommitId(pub String);

impl CommitId {
    pub fn new<S: Into<String>>(s: S) -> Result<Self, CommitIdError> {
        let s = s.into();
        let len = s.len();
        let ok_len = (7..=40).contains(&len) || len == 64;
        if !ok_len || !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(CommitIdError::Invalid);
        }
        Ok(Self(s))
    }

    /// Short version (first 7 chars) of the commit ID.
    pub fn short(&self) -> &str {
        &self.0[..7]
    }
}

impl Deref for CommitId {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CommitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for CommitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.short().fmt(f)
    }
}

impl FromStr for CommitId {
    type Err = CommitIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommitIdError {
    #[error("invalid git object id")]
    Invalid,
}
