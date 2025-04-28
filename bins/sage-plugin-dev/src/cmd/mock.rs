use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use crate::util::*;
use sage_plugin_api::{Event, PluginManager};

#[derive(Args)]
pub struct Mock {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Function to mock (pre-push, post-commit, or run)
    #[arg(short, long)]
    pub function: String,

    /// Mock scenario (normal, empty, long, special-chars, error)
    #[arg(short, long, default_value = "normal")]
    pub scenario: String,

    /// Show debug information
    #[arg(short, long)]
    pub debug: bool,
}

impl Mock {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Check if the plugin has the requested function
        let functions = plugin_manager.get_plugin_functions(&plugin_name)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&self.function) {
            return Err(anyhow!("Plugin does not support {} function", self.function));
        }

        println!("Testing function: {} with scenario: {}", self.function, self.scenario);
        println!("Plugin: {}", plugin_name);

        match self.function.as_str() {
            "pre-push" | "pre_push" => self.mock_pre_push(&mut plugin_manager, &plugin_name)?,
            "post-commit" | "post_commit" => self.mock_post_commit(&mut plugin_manager, &plugin_name)?,
            "run" => self.mock_run(&mut plugin_manager, &plugin_name)?,
            _ => return Err(anyhow!("Unsupported function: {}", self.function)),
        }

        Ok(())
    }

    fn mock_pre_push(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Generate mock branch name based on scenario
        let branch = match self.scenario.as_str() {
            "normal" => "feature/add-new-feature",
            "empty" => "",
            "long" => "feature/extremely-long-branch-name-that-exceeds-normal-limits-and-might-cause-issues-with-some-plugins-that-dont-handle-long-inputs-correctly",
            "special-chars" => "feature/special-chars-!@#$%^&*()_+{}[]|\\:;\"'<>,.?/~`",
            "error" => "error",
            _ => return Err(anyhow!("Unsupported scenario: {}", self.scenario)),
        };

        println!("Testing pre-push with branch: {}", branch);

        // Create the event
        let event = Event::PrePush { branch: branch.to_string() };

        if self.debug {
            println!("DEBUG: Sending pre-push event to plugin");
            println!("DEBUG: Branch: {}", branch);
            println!("DEBUG: Event JSON: {}", serde_json::to_string_pretty(&event)?);
        }

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &[branch.to_string()]) {
            Ok(output) => {
                println!("✅ Plugin execution successful (in {:?})", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("Response: {}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Response: {}", output);
                }
            },
            Err(e) => {
                println!("❌ Plugin execution failed (in {:?})", start_time.elapsed());
                println!("Error: {}", e);
            },
        }

        Ok(())
    }

    fn mock_post_commit(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Generate mock commit ID based on scenario
        let commit_id = match self.scenario.as_str() {
            "normal" => "abcdef1234567890abcdef1234567890abcdef12",
            "empty" => "",
            "long" => "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            "special-chars" => "abcdef!@#$%^&*()_+{}[]|\\:;\"'<>,.?/~`",
            "error" => "error",
            _ => return Err(anyhow!("Unsupported scenario: {}", self.scenario)),
        };

        println!("Testing post-commit with commit ID: {}", commit_id);

        // Create the event
        let event = Event::PostCommit { oid: commit_id.to_string() };

        if self.debug {
            println!("DEBUG: Sending post-commit event to plugin");
            println!("DEBUG: Commit ID: {}", commit_id);
            println!("DEBUG: Event JSON: {}", serde_json::to_string_pretty(&event)?);
        }

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &[commit_id.to_string()]) {
            Ok(output) => {
                println!("✅ Plugin execution successful (in {:?})", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("Response: {}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Response: {}", output);
                }
            },
            Err(e) => {
                println!("❌ Plugin execution failed (in {:?})", start_time.elapsed());
                println!("Error: {}", e);
            },
        }

        Ok(())
    }

    fn mock_run(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Generate mock args based on scenario
        let args = match self.scenario.as_str() {
            "normal" => vec!["arg1".to_string(), "arg2".to_string()],
            "empty" => vec![],
            "long" => vec!["extremely-long-argument-that-exceeds-normal-limits-and-might-cause-issues-with-some-plugins-that-dont-handle-long-inputs-correctly".to_string()],
            "special-chars" => vec!["!@#$%^&*()_+{}[]|\\:;\"'<>,.?/~`".to_string()],
            "error" => vec!["error".to_string()],
            _ => return Err(anyhow!("Unsupported scenario: {}", self.scenario)),
        };

        println!("Testing run with args: {:?}", args);

        if self.debug {
            println!("DEBUG: Executing plugin CLI command");
            println!("DEBUG: Plugin: {}", plugin_name);
            println!("DEBUG: Arguments: {:?}", args);
        }

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &args) {
            Ok(output) => {
                println!("✅ Plugin execution successful (in {:?})", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("Response: {}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Response: {}", output);
                }
            },
            Err(e) => {
                println!("❌ Plugin execution failed (in {:?})", start_time.elapsed());
                println!("Error: {}", e);
            },
        }

        Ok(())
    }
}
