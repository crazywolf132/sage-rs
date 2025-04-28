use crate::{cmd::GitCmd, parse};
use sage_core::{
    error::{CoreError, Result},
    model::{BranchName, Commit, CommitId, CommitMessage, GitAction, MergeResult},
    port::git::{GitExecutor, GitRepo},
};

pub struct ShellGit {
    root: std::path::PathBuf,
}

impl ShellGit {
    pub fn open<P: AsRef<std::path::Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    // ───────────────────────────────── Internal Methods ─────────────────────────────────

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<BranchName> {
        let out = GitCmd::new(&self.root, &["symbolic-ref", "--short", "HEAD"])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git command failed: {:?}", e)))?;

        let branch_name = String::from_utf8_lossy(&out).trim().to_string();
        BranchName::new(branch_name).map_err(|e| CoreError::InvalidOp(format!("Invalid branch name: {:?}", e)))
    }

    /// Create a commit with the given message
    pub fn commit(&self, message: CommitMessage, all: bool, amend: bool) -> Result<()> {
        let mut args = vec!["commit"];

        if all {
            args.push("--all");
        }

        if amend {
            args.push("--amend");
        }

        args.push("-m");
        args.push(message.as_ref());

        GitCmd::new(&self.root, &args)
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git commit failed: {:?}", e)))?;

        Ok(())
    }

    /// Rebase a branch onto a new base
    pub fn rebase(&self, branch: BranchName, new_base: CommitId) -> Result<()> {
        // First checkout the branch to rebase
        GitCmd::new(&self.root, &["checkout", &branch.to_string()])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git checkout failed: {:?}", e)))?;

        // Then perform the rebase
        GitCmd::new(&self.root, &["rebase", &new_base.0])
            .run()
            .map_err(|e| {
                // Try to abort the rebase if it failed
                let _ = GitCmd::new(&self.root, &["rebase", "--abort"]).run();
                CoreError::InvalidOp(format!("Git rebase failed: {:?}", e))
            })?;

        Ok(())
    }

    /// Squash commits from one commit to another
    pub fn squash(&self, from: CommitId, to: CommitId, message: CommitMessage) -> Result<()> {
        // Create a temporary branch at the 'to' commit
        let temp_branch = format!("temp-squash-{}", chrono::Utc::now().timestamp());

        // Create the temp branch
        GitCmd::new(&self.root, &["branch", &temp_branch, &to.0])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git branch creation failed: {:?}", e)))?;

        // Checkout the temp branch
        GitCmd::new(&self.root, &["checkout", &temp_branch])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git checkout failed: {:?}", e)))?;

        // Perform the squash using reset and commit
        GitCmd::new(&self.root, &["reset", "--soft", &from.0])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git reset failed: {:?}", e)))?;

        // Commit with the new message
        GitCmd::new(&self.root, &["commit", "--all", "-m", message.as_ref()])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git commit failed: {:?}", e)))?;

        // Get the new commit hash
        let out = GitCmd::new(&self.root, &["rev-parse", "HEAD"])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git rev-parse failed: {:?}", e)))?;

        let new_commit = String::from_utf8_lossy(&out).trim().to_string();

        // Checkout the original branch
        let current = self.current_branch()?;
        GitCmd::new(&self.root, &["checkout", &current.to_string()])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git checkout failed: {:?}", e)))?;

        // Cherry-pick the new commit
        GitCmd::new(&self.root, &["cherry-pick", &new_commit])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git cherry-pick failed: {:?}", e)))?;

        // Delete the temporary branch
        GitCmd::new(&self.root, &["branch", "-D", &temp_branch])
            .run()
            .map_err(|e| CoreError::InvalidOp(format!("Git branch deletion failed: {:?}", e)))?;

        Ok(())
    }
}

impl GitExecutor for ShellGit {
    fn run_actions(&self, actions: &[GitAction]) -> Result<()> {
        for act in actions {
            match act {
                GitAction::CreateBranch { name, start_port } =>
                    self.create_branch(name.clone(), start_port.clone())?,

                GitAction::Commit { message, all, amend } =>
                    self.commit(message.clone(), *all, *amend)?,

                GitAction::Rebase { branch, new_base } =>
                    self.rebase(branch.clone(), new_base.clone())?,

                GitAction::Squash { from, to, message } =>
                    self.squash(from.clone(), to.clone(), message.clone())?,
            }
        }
        Ok(())
    }
}

impl GitRepo for ShellGit {
    // ───────────────────────────────── Repository Methods ─────────────────────────────────

    fn head(&self) -> Result<CommitId> {
        let out = GitCmd::new(&self.root, &["rev-parse", "HEAD"]).run()
            .map_err(|e| CoreError::InvalidOp(format!("Git command failed: {:?}", e)))?;

        let hash = String::from_utf8_lossy(&out).trim().to_string();
        CommitId::new(hash).map_err(|e| CoreError::InvalidOp(format!("Invalid commit ID: {:?}", e)))
    }

    fn current_branch(&self) -> Result<BranchName> {
        // Reuse the implementation from the ShellGit struct
        self.current_branch()
    }

    // ───────────────────────────────── Commit Methods ─────────────────────────────────

    fn list_commits(
        &self,
        head: CommitId,
        limit: usize,
    ) -> Result<Vec<Commit>> {
        // Format: hash|subject|author|timestamp|body
        let format = "%H|%s|%an|%aI|%b";
        let raw = GitCmd::new(
            &self.root,
            &["log", &head.0, &format!("-n{limit}"), &format!("--pretty={format}")],
        ).run().map_err(|e| CoreError::InvalidOp(format!("Git command failed: {:?}", e)))?;

        let vec = parse::commit_list(std::str::from_utf8(&raw).unwrap())
            .map_err(|e| CoreError::InvalidOp(format!("Parse error: {:?}", e)))?
            .into_iter()
            .map(|(id, subj, author, time_str, body)| {
                // Parse the ISO 8601 timestamp
                let time = chrono::DateTime::parse_from_rfc3339(&time_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                Commit {
                    id: CommitId(id),
                    subject: subj,
                    author,
                    time,
                    body,
                }
            })
            .collect();
        Ok(vec)
    }

    fn commit(&self, message: CommitMessage, all: bool, amend: bool) -> Result<()> {
        // Reuse the implementation from the ShellGit struct
        self.commit(message, all, amend)
    }

    fn squash(&self, from: CommitId, to: CommitId, message: CommitMessage) -> Result<()> {
        // Reuse the implementation from the ShellGit struct
        self.squash(from, to, message)
    }

    // ───────────────────────────────── Branch Methods ─────────────────────────────────

    fn create_branch(&self, name: BranchName, start: CommitId) -> Result<()> {
        GitCmd::new(
            &self.root,
            &["branch", &name.to_string(), &start.0],
        )
        .run()
        .map_err(|e| CoreError::InvalidOp(format!("Git command failed: {:?}", e)))?;

        Ok(())
    }

    fn switch_branch(&self, name: BranchName) -> Result<()> {
        GitCmd::new(
            &self.root,
            &["checkout", &name.to_string()],
        )
        .run()
        .map_err(|e| CoreError::InvalidOp(format!("Git checkout failed: {:?}", e)))?;

        Ok(())
    }

    fn delete_branch(&self, name: BranchName) -> Result<()> {
        GitCmd::new(
            &self.root,
            &["branch", "-D", &name.to_string()],
        )
        .run()
        .map_err(|e| CoreError::InvalidOp(format!("Git delete branch failed: {:?}", e)))?;

        Ok(())
    }

    fn merge(&self, ours: &BranchName, theirs: &BranchName) -> Result<MergeResult> {
        // First checkout our branch
        GitCmd::new(
            &self.root,
            &["checkout", &ours.to_string()],
        )
        .run()
        .map_err(|e| CoreError::InvalidOp(format!("Git checkout failed: {:?}", e)))?;

        // Then try to merge
        let result = GitCmd::new(
            &self.root,
            &["merge", &theirs.to_string()],
        )
        .run();

        match result {
            Ok(_) => Ok(MergeResult::FastForward), // Assuming success is a fast-forward
            Err(_) => {
                // Abort the merge
                let _ = GitCmd::new(
                    &self.root,
                    &["merge", "--abort"],
                )
                .run();

                Ok(MergeResult::Conflict)
            }
        }
    }

    fn rebase(&self, branch: BranchName, new_base: CommitId) -> Result<()> {
        // Reuse the implementation from the ShellGit struct
        self.rebase(branch, new_base)
    }
}