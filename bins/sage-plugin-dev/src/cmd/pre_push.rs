use anyhow::Result;
use clap::Args;
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct PrePush {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Branch name to use in the test
    #[arg(short, long, default_value = "main")]
    pub branch: String,

    /// Show detailed output from the plugin
    #[arg(short, long)]
    pub verbose: bool,

    /// Show debug information
    #[arg(short, long)]
    pub debug: bool,
}

impl PrePush {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Check if the plugin has the pre_push function
        let functions = plugin_manager.get_plugin_functions(&plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&"pre_push".to_string()) {
            println!("⚠️  Warning: Plugin does not support pre_push function");
            return Ok(());
        }

        // Simulate a pre-push event
        println!("Simulating pre-push event for branch: {}", self.branch);
        println!("Plugin: {}", plugin_name);

        match test_pre_push(&mut plugin_manager, &plugin_name, &self.branch, self.debug) {
            Ok(result) => {
                if result.contains("Success") {
                    println!("✅ Pre-push check passed");
                    if self.verbose {
                        println!("\nPlugin output:");
                        println!("{}", result);
                    }
                } else {
                    println!("❌ Pre-push check failed");
                    println!("\nPlugin output:");
                    println!("{}", result);
                }
            },
            Err(e) => {
                println!("❌ Error executing pre-push hook: {}", e);
            }
        }

        Ok(())
    }
}
