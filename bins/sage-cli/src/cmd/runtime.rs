use sage_core::model::UndoLedger;
use sage_git::ShellGit;
use sage_stack::{StackFacade, CreateChildOpts};
use sage_plugin_api::{PluginManager, PluginRegistry};
use crate::util::EchoAI;
use anyhow::Result;
use sage_core::model::GitAction;

pub struct Runtime {
    pub git: ShellGit,
    pub ai: EchoAI,
    pub ledger: UndoLedger,
    pub plugins: PluginRegistry,
}

impl Runtime {
    pub fn init(git: ShellGit) -> Result<Self> {
        let ai = EchoAI;
        let ledger = UndoLedger::default();
        // Expand the home directory
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
        let plugin_dir = format!("{}/.config/sage/plugins", home_dir);
        let plugin_manager = PluginManager::load_dir(&plugin_dir)?;
        let plugins = PluginRegistry::new(plugin_manager);

        Ok(Self { git, ai, ledger, plugins })
    }

    pub fn create_child(&mut self, opts: CreateChildOpts) -> Result<Vec<GitAction>> {
        let mut stack = StackFacade::new(&self.git, &self.ai, &mut self.ledger);
        stack.create_child(opts)
    }

    pub fn undo(&mut self) -> Result<()> {
        let mut stack = StackFacade::new(&self.git, &self.ai, &mut self.ledger);
        stack.undo()?;
        Ok(())
    }

    pub fn redo(&mut self) -> Result<()> {
        if let Some(_op) = self.ledger.redo() {
            // TODO: Implement proper handling of redo operation
            // This is similar to how undo would handle ops
            Ok(())
        } else {
            anyhow::bail!("Nothing to redo")
        }
    }
}