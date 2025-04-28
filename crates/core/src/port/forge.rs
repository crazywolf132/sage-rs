//! Minimal forge trait - GitHub/GitLab/Bitbucket adapters will implement.

use crate::error::CoreError;
use crate::model::{BranchName, CommitId};

#[derive(Debug)]
pub struct PullRequest {
    pub url: String,
    pub number: u64,
    pub title: String,
}

pub trait Forge {
    fn create_pr(
        &self,
        branch: &BranchName,
        base: &BranchName,
        title: &str,
        body: &str,
    ) -> Result<PullRequest, CoreError>;

    fn get_pr(&self, number: u64) -> Result<PullRequest, CoreError>;

    fn merge_pr(&self, number: u64) -> Result<(), CoreError>;
}
