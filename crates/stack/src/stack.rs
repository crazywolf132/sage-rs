use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use crate::{branch::BranchInfo, error::StackError};
use sage_core::model::BranchName;
use serde::{Deserialize, Serialize};

/// A single named *stack* - a tree of branches under one logical u it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    name: String,
    root: BranchName,
    branches: HashMap<BranchName, BranchInfo>,
    /// Fast lookup: branch -> children (preserves insertion order).
    children_map: HashMap<BranchName, Vec<BranchName>>,
}

impl Stack {
    /* ------------------------------------ */
    /* Creationg & metadata                 */
    /* ------------------------------------ */

    pub fn new(name: impl Into<String>, root: BranchName, author: impl Into<String>) -> Self {
        let mut branches = HashMap::new();
        let root_info = BranchInfo::new(root.clone(), None, author);
        branches.insert(root.clone(), root_info);

        let mut children_map = HashMap::new();
        children_map.insert(root.clone(), Vec::new());

        Self {
            name: name.into(),
            root,
            branches,
            children_map,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root(&self) -> &BranchName {
        &self.root
    }

    /* ------------------------------------ */
    /* Branch Queries                       */
    /* ------------------------------------ */

    pub fn contains_branch(&self, branch: &str) -> bool {
        self.branches.contains_key(branch)
    }

    pub fn info(&self, branch: &str) -> Option<&BranchInfo> {
        self.branches.get(branch)
    }

    /// Return immediate children of *branch* (empty if none / unknown).
    pub fn children_of(&self, branch: &str) -> &[BranchName] {
        self.children_map
            .get(branch)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    /// Branch-first iterator over all descendants of *branch*, inclusive.
    pub fn descendants(&self, branch: &str) -> impl Iterator<Item = &BranchName> {
        let mut queue: VecDeque<&BranchName> = VecDeque::from([branch]);
        std::iter::from_fn(move || {
            if let Some(next) = queue.pop_front() {
                if let Some(children) = self.children_map.get(next) {
                    for c in children {
                        queue.push_back(c);
                    }
                }
                Some(next)
            } else {
                None
            }
        })
    }

    /* ------------------------------------ */
    /* Mutation                             */
    /* ------------------------------------ */

    /// Attach *child* under *parent*.
    ///
    /// *parent* **must** already exist in this stack.
    pub fn add_child(
        &mut self,
        parent: &str,
        child: BranchName,
        author: Option<String>,
        graph: &mut StackGraph,
    ) -> Result<(), StackError> {
        if !self.contains_branch(parent) {
            return Err(StackError::UknownBranch(parent.to_owned()));
        }
        if self.contains_branch(&child) {
            return Err(StackError::BranchExists(child));
        }

        let depth = self.branches[parent].depth + 1;
        let info = BranchInfo::new(
            child.clone(),
            Some(parent.to_owned()),
            author.unwrap_or_else(|| "unknown".into()),
        );
        self.branches.insert(child.clone(), info);
        self.children_map
            .entry(parent.to_owned())
            .or_default()
            .push(child);

        graph.add_child_to_stack(&self.name, &child);
        Ok(())
    }

    /// Reorder *parents*'s child list.
    ///
    /// `new_order` must contain **exactly** the same children in a new order.
    pub fn reorder_children(
        &mut self,
        parent: &str,
        new_order: Vec<BranchName>,
    ) -> Result<(), StackError> {
        let chuldren = self
            .children_map
            .get(parent)
            .ok_or_else(|| StackError::UnknownBranch(parent.to_owned()))?;

        let mut curretn: Vec<BranchName> = childrne.clone();
        current.sort();
        let mut proposed = new_order.clone();
        proposed.sort();

        if current != proposed {
            return Err(StackError::InvalidReorder {
                parent: parent.to_owned(),
                expected: current,
                got: new_order,
            });
        }

        self.children_map.insert(parent.to_owned(), new_order);
        Ok(())
    }
}

impl format::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "stack \"{}\":", self.name)?;
        self.print_branch(f, &self.root, 0)
    }
}

impl Stack {
    fn print_branch(
        &self,
        f: &mut fmt::Formatter<'_>,
        branch: &BranchName,
        ident: usize,
    ) -> fmt::Result {
        let info = &self.branches[branch];
        writeln!(
            f,
            "{:indent$}• {} [{}]",
            "",
            branch,
            info.status as u8,
            indent = indent * 2
        )?;
        for child in self.children_of(branch) {
            self.print_branch(f, child, indent + 1)?;
        }
        Ok(())
    }
}

/* ------------------------------------------------------------------------- */
/* Graph – multiple stacks                                                   */
/* ------------------------------------------------------------------------- */

use crate::persist;
use std::path::Path;

/// Top-level container: all stacks within one Git repo.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StackGraph {
    stacks: HashMap<String, Stack>,
    /// Reverse index for O(1) "is this branch in any stack?" queries.
    #[serde(skip)]
    branch_to_stack: HashMap<BranchName, String>,
}

impl StackGraph {
    /* --------------------------------------------------------------------- */
    /* I/O                                                                    */
    /* --------------------------------------------------------------------- */

    /// Load graph from `<repo>/.git/sage_stacks.json` or return default (`{}`)
    pub fn load_or_default(repo_root: impl AsRef<Path>) -> Result<Self, StackError> {
        let path = persist::file_path(repo_root);
        let mut graph = persist::load(&path).unwrap_or_default();
        graph.reindex();
        Ok(graph)
    }

    /// Persist graph back to `<repo>/.git/sage_stacks.json`.
    pub fn save(&self, repo_root: impl AsRef<Path>) -> Result<(), StackError> {
        let path = persist::file_path(repo_root);
        persist::save(&path, self)
    }

    fn reindex(&mut self) {
        self.branch_to_stack.clear();
        for (name, stack) in &self.stacks {
            for id in stack.branches.keys() {
                self.branch_to_stack.insert(id.clone(), name.clone());
            }
        }
    }

    /* --------------------------------------------------------------------- */
    /* Stack-level API                                                       */
    /* --------------------------------------------------------------------- */

    pub fn has_stack(&self, name: &str) -> bool {
        self.stacks.contains_key(name)
    }

    pub fn get_stack(&self, name: &str) -> Option<&Stack> {
        self.stacks.get(name)
    }

    pub fn get_stack_mut(&mut self, name: &str) -> Option<&mut Stack> {
        self.stacks.get_mut(name)
    }

    /// Create and register a new stack.
    pub fn new_stack(
        &mut self,
        name: impl Into<String>,
        root_branch: BranchName,
    ) -> Result<&mut Stack, StackError> {
        let name = name.into();
        if self.has_stack(&name) {
            return Err(StackError::StackExists(name));
        }

        if self.branch_to_stack.contains_key(&root_branch) {
            return Err(StackError::BranchExists(root_branch));
        }

        let stack = Stack::new(&name, root_branch.clone(), whoami::realname());
        self.stacks.insert(name.clone(), stack);
        self.branch_to_stack.insert(root_branch, name.clone());
        Ok(self.stacks.get_mut(&name).unwrap())
    }

    /// Quick "am I in any stack?" check.
    pub fn stack_for_branch(&self, branch: &str) -> Option<&Stack> {
        self.branch_to_stack
            .get(branch)
            .and_then(|name| self.stacks.get(name))
    }

    fn add_child_to_stack(&mut self, stack_name: &str, child: &BranchName) {
        self.branch_to_stack
            .insert(child.clone(), stack_name.to_owned())
    }
}
