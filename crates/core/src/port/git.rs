use crate::model::*;

// ───────────────────────────────── Git Repository Interface ─────────────────────────────────

pub trait GitRepo {
    // Repository methods
    fn head(&self) -> crate::error::Result<CommitId>;
    fn current_branch(&self) -> crate::error::Result<BranchName>;

    // Commit methods
    fn list_commits(&self, head: CommitId, limit: usize) -> crate::error::Result<Vec<Commit>>;
    fn commit(&self, message: CommitMessage, all: bool, amend: bool) -> crate::error::Result<()>;
    fn squash(&self, from: CommitId, to: CommitId, message: CommitMessage) -> crate::error::Result<()>;

    // Branch methods
    fn create_branch(&self, name: BranchName, start: CommitId) -> crate::error::Result<()>;
    fn switch_branch(&self, name: BranchName) -> crate::error::Result<()>;
    fn delete_branch(&self, name: BranchName) -> crate::error::Result<()>;
    fn merge(&self, ours: &BranchName, theirs: &BranchName) -> crate::error::Result<MergeResult>;
    fn rebase(&self, branch: BranchName, new_base: CommitId) -> crate::error::Result<()>;
}

// ───────────────────────────────── Git Action Executor ─────────────────────────────────

pub trait GitExecutor {
    fn run_actions(&self, actions: &[GitAction]) -> crate::error::Result<()>;
}
