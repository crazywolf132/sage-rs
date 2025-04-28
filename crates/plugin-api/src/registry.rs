use std::sync::{Arc, Mutex};

use crate::{PluginError, PluginManager, Result};

/// A registry for plugins that can be shared across the application.
/// This provides a centralized way to access plugins from different parts of the code.
#[derive(Clone)]
pub struct PluginRegistry {
    manager: Arc<Mutex<PluginManager>>,
}

impl PluginRegistry {
    /// Create a new plugin registry with the given plugin manager.
    pub fn new(manager: PluginManager) -> Self {
        Self {
            manager: Arc::new(Mutex::new(manager)),
        }
    }

    /// Get a reference to the plugin manager.
    pub fn manager(&self) -> Result<std::sync::MutexGuard<'_, PluginManager>> {
        self.manager.lock().map_err(|_| PluginError::Plugin("Failed to lock plugin manager".into()))
    }

    /// Run a plugin's CLI command.
    pub fn run_cli(&self, plugin: &str, args: &[String]) -> Result<String> {
        let mut manager = self.manager()?;
        manager.cli(plugin, args)
    }

    /// Run the pre-push hook for all plugins.
    pub fn run_pre_push(&self, branch: &str) -> Result<()> {
        let mut manager = self.manager()?;
        manager.pre_push(branch)
    }

    /// Run the post-commit hook for all plugins.
    pub fn run_post_commit(&self, oid: &str) {
        if let Ok(mut manager) = self.manager() {
            manager.post_commit(oid);
        }
    }

    /// List all installed plugins.
    pub fn list_plugins(&self) -> Result<Vec<String>> {
        let manager = self.manager()?;
        manager.list_plugins()
    }

    /// Install a plugin from a file.
    pub fn install_plugin(&self, path: &str) -> Result<()> {
        let mut manager = self.manager()?;
        manager.install_plugin(path)
    }

    /// Check if a plugin exists.
    pub fn has_plugin(&self, name: &str) -> Result<bool> {
        let manager = self.manager()?;
        Ok(manager.has_plugin(name))
    }
}
