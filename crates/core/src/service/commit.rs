use crate::{
    error::{CoreError, Result},
    model::{CommitId, CommitMessage, DiffStats, GitAction, LintFailure, StackConfig},
    port::{ai::AiAssistant, git::GitRepo},
};

#[derive(Clone, Debug)]
pub struct CommitConfig {
    pub require_conventional: bool,
    pub auto_add: bool,
}

pub struct CommitService<'r, R: GitRepo, A: AiAssistant> {
    repo: &'r R,
    ai: &'r A,
    cfg: CommitConfig,
}

impl<'r, R: GitRepo, A: AiAssistant> CommitService<'r, R, A> {
    pub fn new(repo: &'r R, ai: &'r A, cfg: CommitConfig) -> Self {
        Self { repo, ai, cfg }
    }

    fn lint(&self, m: &CommitMessage) -> Result<()> {
        if self.cfg.require_conventional && !m.is_conventional() {
            Err(CoreError::Lint(LintFailure::NonConventional))
        } else {
            Ok(())
        }
    }

    pub fn create_commit(&self, msg: CommitMessage, staged_only: bool) -> Result<Vec<GitAction>> {
        self.lint(&msg)?;
        Ok(vec![GitAction::Commit {
            message: msg,
            all: self.cfg.auto_add && !staged_only,
            amend: false,
        }])
    }

    pub fn amend_last(&self, new_msg: Option<CommitMessage>) -> Result<Vec<GitAction>> {
        if let Some(ref m) = new_msg {
            self.lint(m)?;
        }
        Ok(vec![GitAction::Commit {
            message: new_msg.unwrap_or_default(),
            all: self.cfg.auto_add,
            amend: true,
        }])
    }

    pub fn generate_message(&self, diff: &DiffStats) -> Result<CommitMessage> {
        let prompt = format!(
            "Generate a Conventional Commit message:\nadded {}, removed {}, files {}",
            diff.added, diff.removed, diff.files
        );
        let raw = self.ai.generate_text(&prompt, 60)?;
        Ok(CommitMessage::new(raw))
    }

    pub fn squash_range(
        &self,
        from: CommitId,
        to: CommitId,
        msg: CommitMessage,
    ) -> Result<Vec<GitAction>> {
        self.lint(&msg)?;
        Ok(vec![GitAction::Squash {
            from,
            to,
            message: msg,
        }])
    }
}
