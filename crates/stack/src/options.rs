//! Simple option structs used by the facade so the CLI can convert clap args
//! straight into type-checked data.

use sage_core::model::{BranchName, CommitMessage};

#[derive(Clone, Debug)]
pub struct CreateChildOpts {
    pub name: BranchName,
}

#[derive(Clone, Debug)]
pub struct RestackOpts {
    pub branch: BranchName,
}

#[derive(Clone, Debug)]
pub struct CommitOpts {
    pub message: CommitMessage,
    pub staged_only: bool,
}