use super::{branch::BranchName, CommitId, CommitMessage};

#[derive(Debug)]
pub enum GitAction {
    CreateBranch {
        name: BranchName,
        start_port: CommitId,
    },
    Commit {
        message: CommitMessage,
        all: bool,
        amend: bool,
    },
    Rebase {
        branch: BranchName,
        new_base: CommitId,
    },
    Squash {
        from: CommitId,
        to: CommitId,
        message: CommitMessage,
    },
}