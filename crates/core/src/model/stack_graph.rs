//! A lightweight DAG representing stacked branches.
//
// - Nodes: branch name + tip commit.
// - Edges: parent branch -> child branch (linear chain typical).
// - Operations: add_child, parent_of, next/prev, rebase base calculation.

use super::{BranchName, CommitId};
use crate::error::CoreError;
use std::collections::{HashMap, HashSet};

/// Branch metadata stored in each node.
#[derive(Clone, Debug)]
pub struct Node {
    pub branch: BranchName,
    pub commit: CommitId,
}

#[derive(Clone, Debug, Default)]
pub struct StackGraph {
    /// branch -> parent branch (none == root)
    parents: HashMap<BranchName, Option<BranchName>>,
    /// branch -> children
    children: HashMap<BranchName, Vec<BranchName>>,
    /// branch -> tip commit
    commits: HashMap<BranchName, CommitId>,
}

impl StackGraph {
    /// Build a **best‑effort** stack graph from a chronological history.
    ///
    /// Strategy ­­­
    /// 1. Start with a virtual root branch for the **first** commit.
    /// 2. As we walk the rest of the commits:
    ///    * If the commit **already exists** as the tip of some branch we
    ///      assume it’s the *parent* of a new child branch and fork there.
    ///    * Otherwise we extend the **previous** branch (linear case).
    ///
    /// This simple heuristic supports multiple children (fan‑out) while
    /// keeping the algorithm O(n) and independent of libgit².
    pub fn from_commits<I>(history: I) -> Result<Self, CoreError>
    where
        I: IntoIterator<Item = crate::model::Commit>,
    {
        let mut iter = history.into_iter();
        let first = iter.next().ok_or_else(|| CoreError::Graph("empty history".into()))?;

        let mut g = StackGraph::default();
        let mut current_branch = BranchName::new("root".to_string())
            .map_err(|_| CoreError::Graph("invalid branch name".into()))?;

        // insert root node
        g.parents.insert(current_branch.clone(), None);
        g.children.insert(current_branch.clone(), Vec::new());
        g.commits.insert(current_branch.clone(), first.id.clone());

        for commit in iter {
            // If any branch already ends at this commit we are at a **fork** –
            // create a child branch of that tip.
            if let Some((parent_branch, _)) =
                g.commits.iter().find(|(_, tip)| *tip == &commit.id)
            {
                let child_idx = g.children.get(parent_branch).unwrap().len() + 1;
                current_branch = BranchName::new(format!("{}/{}", parent_branch, child_idx))
                    .map_err(|_| CoreError::Graph("invalid branch name".into()))?;

                g.parents
                    .insert(current_branch.clone(), Some(parent_branch.clone()));
                g.children
                    .entry(parent_branch.clone())
                    .or_default()
                    .push(current_branch.clone());
                g.children.insert(current_branch.clone(), Vec::new());
            }
            // otherwise continue on current branch (linear extension)

            // record tip
            g.commits.insert(current_branch.clone(), commit.id.clone());
        }
        Ok(g)
    }

    pub fn current_tip(&self) -> Option<Node> {
        // tip = branch with no children
        let tip = self.children.iter().find(|(_, v)| v.is_empty())?.0.clone();
        Some(Node {
            branch: tip.clone(),
            commit: self.commits.get(&tip)?.clone(),
        })
    }

    /// Add `child` as new branch under `tip_commit`.
    pub fn add_child(&mut self, tip_commit: &CommitId, child: BranchName) -> Result<(), CoreError> {
        // Ensure no cycles, duplicates
        if self.commits.contains_key(&child) {
            return Err(CoreError::Graph("branch already exists".into()));
        }

        // Find branch that owns tip_commit
        let parent = self
            .commits
            .iter()
            .find(|(_, c)| *c == tip_commit)
            .map(|(b, _)| b.clone())
            .ok_or_else(|| CoreError::Graph("parent branch not found".into()))?;

        self.parents.insert(child.clone(), Some(parent.clone()));
        self.children.entry(parent).or_default().push(child.clone());
        self.children.insert(child.clone(), Vec::new());
        self.commits.insert(child, tip_commit.clone());
        Ok(())
    }

    // Relationships -----------------------------------------------------------

    pub fn parent_of(&self, b: &BranchName) -> Option<Node> {
        let p = self.parents.get(b)?.clone()?;
        Some(Node {
            branch: p.clone(),
            commit: self.commits.get(&p)?.clone(),
        })
    }
    pub fn next_of(&self, b: &BranchName) -> Option<BranchName> {
        let parent = self.parents.get(b)?.clone()?;
        let siblings = self.children.get(&parent)?;
        let idx = siblings.iter().position(|x| x == b)?;
        siblings.get(idx + 1).cloned()
    }

    pub fn prev_of(&self, b: &BranchName) -> Option<BranchName> {
        let parent = self.parents.get(b)?.clone()?;
        let siblings = self.children.get(&parent)?;
        let idx = siblings.iter().position(|x| x == b)?;
        if idx == 0 {
            None
        } else {
            siblings.get(idx - 1).cloned()
        }
    }
}

/// Result of a merge attempted by the Git adapter.
#[derive(Clone, Copy, Debug)]
pub enum MergeResult {
    /// Fast-forward merge (no changes)
    FastForward,
    /// No-op merge (no changes)
    NoOp,
    /// Conflicting merge (manual resolution required)
    Conflict,
}
