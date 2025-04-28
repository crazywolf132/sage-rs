use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use crate::util::*;
use sage_plugin_api::{Event, PluginManager};

#[derive(Args)]
pub struct Trace {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Function to trace (pre-push, post-commit, or run)
    #[arg(short, long)]
    pub function: String,

    /// Branch name for pre-push events
    #[arg(short, long, default_value = "main")]
    pub branch: String,

    /// Commit ID for post-commit events
    #[arg(short, long, default_value = "abcdef1234567890")]
    pub commit_id: String,

    /// Arguments for run function
    #[arg(short, long)]
    pub args: Vec<String>,
}

impl Trace {
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

        println!("Tracing function: {}", self.function);
        println!("Plugin: {}", plugin_name);

        match self.function.as_str() {
            "pre-push" | "pre_push" => self.trace_pre_push(&mut plugin_manager, &plugin_name)?,
            "post-commit" | "post_commit" => self.trace_post_commit(&mut plugin_manager, &plugin_name)?,
            "run" => self.trace_run(&mut plugin_manager, &plugin_name)?,
            _ => return Err(anyhow!("Unsupported function: {}", self.function)),
        }

        Ok(())
    }

    fn trace_pre_push(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Create the event
        let event = Event::PrePush { branch: self.branch.clone() };
        let event_json = serde_json::to_string_pretty(&event)?;

        println!("\n📤 Sending event to plugin:");
        println!("{}", event_json);

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &[self.branch.clone()]) {
            Ok(output) => {
                println!("\n📥 Received response from plugin (in {:?}):", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("{}", output);
                }
            },
            Err(e) => {
                println!("\n❌ Error from plugin (in {:?}):", start_time.elapsed());
                println!("{}", e);
            },
        }

        Ok(())
    }

    fn trace_post_commit(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Create the event
        let event = Event::PostCommit { oid: self.commit_id.clone() };
        let event_json = serde_json::to_string_pretty(&event)?;

        println!("\n📤 Sending event to plugin:");
        println!("{}", event_json);

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &[self.commit_id.clone()]) {
            Ok(output) => {
                println!("\n📥 Received response from plugin (in {:?}):", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("{}", output);
                }
            },
            Err(e) => {
                println!("\n❌ Error from plugin (in {:?}):", start_time.elapsed());
                println!("{}", e);
            },
        }

        Ok(())
    }

    fn trace_run(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        println!("\n📤 Sending CLI arguments to plugin:");
        println!("{:?}", self.args);

        // Call the plugin
        let start_time = std::time::Instant::now();
        match plugin_manager.cli(plugin_name, &self.args) {
            Ok(output) => {
                println!("\n📥 Received response from plugin (in {:?}):", start_time.elapsed());

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("{}", output);
                }
            },
            Err(e) => {
                println!("\n❌ Error from plugin (in {:?}):", start_time.elapsed());
                println!("{}", e);
            },
        }

        Ok(())
    }
}
