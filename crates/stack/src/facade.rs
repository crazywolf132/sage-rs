//! A facade that stitches together core service (`StackService`,
//! `CommitService`, `UndoService`) and returns *ready-to-execute* planes for
//! the outer Git/TUI layers.

use sage_core::{
    model::{BranchName, CommitMessage, GitAction, StackConfig, UndoLedger},
    port::{ai::AiAssistant, git::GitRepo},
    service::{commit::{CommitConfig, CommitService}, stack::StackService, undo::UndoService},
};

use crate::options::*;

pub struct StackFacade<'a, R: GitRepo, A: AiAssistant> {
    stack: StackService<'a, R>,
    commit: CommitService<'a, R, A>,
    undo: UndoService<'a>,
}

impl<'a, R: GitRepo, A: AiAssistant> StackFacade<'a, R, A> {
    pub fn new(repo: &'a R, ai: &'a A, ledger: &'a mut UndoLedger) -> Self {
        let cfg = StackConfig::default();
        let stack = StackService::new(repo, cfg.clone());
        let commit_cfg = CommitConfig { require_conventional: true,  auto_add: true };
        let commit = CommitService::new(repo, ai, commit_cfg);
        let undo = UndoService::new(ledger);
        Self { stack, commit, undo}
    }

    /// Wrapps `StackService::create_child_branch` + undo recording.
    pub fn create_child(&mut self, opts: CreateChildOpts) -> anyhow::Result<Vec<GitAction>> {
        let (graph, acts) = self.stack.create_child_branch(&opts.name)?;
        self.undo.record(sage_core::model::UndoOp::CreateBranch { name: opts.name.to_string() });
        Ok(acts)
    }

    pub fn restack(&mut self, opts: RestackOpts) -> anyhow::Result<Vec<GitAction>> {
        let acts = self.stack.restack_branch(&opts.branch)?;
        Ok(acts)
    }

    pub fn commit(&mut self, opts: CommitOpts) -> anyhow::Result<Vec<GitAction>> {
        let acts = self.commit.create_commit(opts.message, opts.staged_only)?;
        Ok(acts)
    }

    pub fn undo(&mut self) -> anyhow::Result<Vec<GitAction>> {
        let op = self.undo.undo()?;
        // TODO: translate UndoOp -> GitAction
        Ok(vec![])
    }
}
