//! `BranchName` – validates Git branch/ref rules (subset of `git check-ref-format`).

use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref, str::FromStr};

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BranchName(String);

impl BranchName {
    pub fn new<S: Into<String>>(s: S) -> Result<Self, BranchNameError> {
        let s = s.into();
        let bad = s.is_empty()
            || s.starts_with('/')
            || s.ends_with('/')
            || s.contains(' ')
            || s.contains(|c: char| matches!(c, '~' | '^' | ':' | '?' | '*' | '[' | '\\'))
            || s.contains("..");
        if bad {
            return Err(BranchNameError::Invalid);
        }
        Ok(Self(s))
    }
}

impl Deref for BranchName {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for BranchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for BranchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for BranchName {
    type Err = BranchNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BranchNameError {
    #[error("invalid branch/ref name")]
    Invalid,
}
