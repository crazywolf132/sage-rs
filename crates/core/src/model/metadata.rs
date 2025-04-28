use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

use super::stack::Stack;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetachedHeadContext {
    pub stack_name: String,
    pub branch_name: String,
}

/// Global storage structure to handle multiple stacks.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SolMetadata {
    pub stacks: Vec<Stack>,
    pub version: String, // For future migration compatibility
    #[serde(default)]
    pub detached_head_context: Option<DetachedHeadContext>,
}

impl SolMetadata {
    pub fn set_detached_head_context(
        &mut self,
        stack_name: String,
        branch_name: String,
    ) -> Result<()> {
        self.detached_head_context = Some(DetachedHeadContext {
            stack_name,
            branch_name,
        });
        Ok(())
    }

    pub fn clear_detached_head_context(&mut self) {
        self.detached_head_context = None;
    }

    pub fn get_detached_head_context(&self) -> Option<&DetachedHeadContext> {
        self.detached_head_context.as_ref()
    }

    pub fn is_in_detached_head(&self) -> bool {
        self.detached_head_context.is_some()
    }
}

// Integrating the `.load()` and `.save()` methods into the `SolMetadata` struct
impl SolMetadata {
    /// save the database file to disk.
    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        let path = Self::get_storage_path()?;
        fs::write(path, data)?;
        Ok(())
    }

    /// load the database file from disk.
    pub fn load() -> Result<Self> {
        let path = Self::get_storage_path()?;
        if !path.exists() {
            return Ok(SolMetadata {
                stacks: vec![],
                version: "0.1.0".to_string(),
                detached_head_context: None,
            });
        }
        let data = fs::read_to_string(path)?;
        let metadata: SolMetadata = serde_json::from_str(&data)?;
        Ok(metadata)
    }

    /// get_storage_path returns the path to the database file.
    fn get_storage_path() -> Result<PathBuf> {
        // Get the git directory using git rev-parse
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--git-dir")
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "Not in a git repository. Please run this command from within a git repository."
            ));
        }

        let git_dir = String::from_utf8(output.stdout)?.trim().to_string();
        let mut path = PathBuf::from(git_dir);
        path.push("sol-metadata.json");
        Ok(path)
    }
}
