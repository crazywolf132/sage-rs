use crate::{PluginRegistry, Result};

/// Helper functions for integrating plugins with git operations.
pub struct GitHooks {
    registry: PluginRegistry,
}

impl GitHooks {
    /// Create a new GitHooks instance.
    pub fn new(registry: PluginRegistry) -> Self {
        Self { registry }
    }

    /// Run the pre-push hook for all plugins.
    /// This should be called before pushing to a remote.
    /// If any plugin returns an error, the push will be blocked.
    pub fn pre_push(&self, branch: &str) -> Result<()> {
        self.registry.run_pre_push(branch)
    }

    /// Run the post-commit hook for all plugins.
    /// This should be called after a commit is created.
    /// Errors are logged but ignored.
    pub fn post_commit(&self, commit_id: &str) {
        self.registry.run_post_commit(commit_id);
    }
}

/// Helper trait for integrating plugins with git operations.
pub trait WithPlugins {
    /// Get a reference to the plugin registry.
    fn plugin_registry(&self) -> &PluginRegistry;

    /// Run the pre-push hook for all plugins.
    fn run_pre_push(&self, branch: &str) -> Result<()> {
        self.plugin_registry().run_pre_push(branch)
    }

    /// Run the post-commit hook for all plugins.
    fn run_post_commit(&self, commit_id: &str) {
        self.plugin_registry().run_post_commit(commit_id);
    }
}
