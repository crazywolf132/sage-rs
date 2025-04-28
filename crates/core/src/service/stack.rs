use crate::{
    error::{CoreError, Result},
    model::{BranchName, CommitId, GitAction, StackConfig, StackGraph},
    port::git::GitRepo,
};

pub struct StackService<'r, R: GitRepo> {
    repo: &'r R,
    config: StackConfig,
}

impl<'r, R: GitRepo> StackService<'r, R> {
    pub fn new(repo: &'r R, config: StackConfig) -> Self {
        Self { repo, config }
    }

    pub fn discover(&self) -> Result<StackGraph> {
        let head = self.repo.head()?;
        let history = self.repo.list_commits(head, self.config.history_depth)?;
        StackGraph::from_commits(history).map_err(Into::into)
    }

    /// `sage stack child <name>`
    pub fn create_child_branch(&self, new: &BranchName) -> Result<(StackGraph, Vec<GitAction>)> {
        let mut graph = self.discover()?;
        let tip = graph
            .current_tip()
            .ok_or(CoreError::Graph("empty repo".into()))?;
        graph.add_child(&tip.commit, new.clone())?;

        let actions = vec![GitAction::CreateBranch {
            name: new.clone(),
            start_port: tip.commit,
        }];
        Ok((graph, actions))
    }

    /// `sage stack restack <name>`
    pub fn restack_branch(&self, b: &BranchName) -> Result<Vec<GitAction>> {
        let graph = self.discover()?;
        let parent = graph
            .parent_of(b)
            .ok_or(CoreError::Graph("no parent".into()))?;
        Ok(vec![GitAction::Rebase {
            branch: b.clone(),
            new_base: parent.commit,
        }])
    }

    pub fn adjacent_branch(&self, current: &BranchName, next: bool) -> Result<BranchName> {
        let graph = self.discover()?;
        match (next, graph.next_of(current), graph.prev_of(current)) {
            (true, Some(n), _) => Ok(n),
            (false, _, Some(p)) => Ok(p),
            _ => Err(CoreError::InvalidOp("end of stack".into())),
        }
    }
}
