//! Commit message new-type with Conventional Commits helper.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref, str::FromStr};

static CC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([\w\-.]+\))?: .+",
    )
    .unwrap()
});

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct CommitMessage(String);

impl CommitMessage {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let mut m = s.into();
        while m.ends_with(['\n', '\r']) {
            m.pop();
        }
        Self(m)
    }
    pub fn summary(&self) -> &str {
        self.0.split('\n').next().unwrap_or_default()
    }
    pub fn is_conventional(&self) -> bool {
        CC_RE.is_match(self.summary())
    }
}

impl Deref for CommitMessage {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.summary().fmt(f)
    }
}
impl FromStr for CommitMessage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
