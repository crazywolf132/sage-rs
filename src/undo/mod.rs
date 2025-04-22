use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{DateTime, Utc};

pub mod service;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum OperationType {
    Commit,
    Merge,
    Rebase,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Commit => write!(f, "commit"),
            OperationType::Merge => write!(f, "merge"),
            OperationType::Rebase => write!(f, "rebase"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Operation {
    pub id: String,
    pub type_: OperationType,
    pub description: String,
    pub command: String,
    pub timestamp: DateTime<Utc>,
    pub ref_: String,
    pub category: String,
    pub metadata: OperationMetadata,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OperationMetadata {
    pub files: Vec<String>,
    pub branch: String,
    pub message: String,
    pub extra: HashMap<String, String>,
    pub stashed: bool,
    pub stash_ref: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct History {
    pub operations: Vec<Operation>,
    pub max_size: usize,
}

impl History {
    pub fn add(&mut self, operation: Operation) -> Result<()> {
        if self.operations.len() >= self.max_size {
            self.operations.remove(0);
        }
        self.operations.push(operation);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Operation> {
        self.operations.iter().find(|op| op.id == id)
    }

    pub fn get_all(&self) -> Vec<&Operation> {
        self.operations.iter().collect()
    }

    pub fn save(&self) -> Result<()> {
        let path = get_history_path()?;
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = get_history_path()?;
        let data = fs::read_to_string(path)?;
        let history = serde_json::from_str(&data)?;
        Ok(history)
    }
}

/// Returns the path to the .git/sage_history file at the root of the current git repo
fn get_history_path() -> Result<PathBuf> {
    // Use our git::repo::git_home function to find the .git directory
    let git_dir = crate::git::repo::git_home()?;
    Ok(git_dir.join("sage_history"))
}