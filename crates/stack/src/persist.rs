//! Tiny helper for (de)serialising the graph JSON.

use std::{fs, path::PathBuf};

use crate::{error::StackError, stack::StackGraph};

pub(crate) fn file_path(repo_root: impl AsRef<std::path::Path>) -> PathBuf {
    repo_root.as_ref().join(".git").join("zyra_stacks.json")
}

pub(crate) fn load(path: &PathBuf) -> Result<StackGraph, StackError> {
    match fs::read_to_string(path) {
        Ok(raw) => Ok(serde_json::from_str(&raw)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(StackGraph::default()),
        Err(e) => Err(StackError::Io(e)),
    }
}

pub(crate) fn save(path: &PathBuf, graph: &StackGraph) -> Result<(), StackError> {
    let raw = serde_json::to_string_pretty(graph)?;
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::Other))?,
    )?;
    fs::write(path, raw)?;
    Ok(())
}
