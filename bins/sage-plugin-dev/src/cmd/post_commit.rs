use anyhow::Result;
use clap::Args;
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct PostCommit {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Commit ID to use in the test
    #[arg(short, long, default_value = "abcdef1234567890")]
    pub commit_id: String,

    /// Show detailed output from the plugin
    #[arg(short, long)]
    pub verbose: bool,

    /// Show debug information
    #[arg(short, long)]
    pub debug: bool,
}

impl PostCommit {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Check if the plugin has the post_commit function
        let functions = plugin_manager.get_plugin_functions(&plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&"post_commit".to_string()) {
            println!("⚠️  Warning: Plugin does not support post_commit function");
            return Ok(());
        }

        // Simulate a post-commit event
        println!("Simulating post-commit event for commit: {}", self.commit_id);
        println!("Plugin: {}", plugin_name);

        match test_post_commit(&mut plugin_manager, &plugin_name, &self.commit_id, self.debug) {
            Ok(result) => {
                if result.contains("Success") {
                    println!("✅ Post-commit check passed");
                    if self.verbose {
                        println!("\nPlugin output:");
                        println!("{}", result);
                    }
                } else {
                    println!("❌ Post-commit check failed");
                    println!("\nPlugin output:");
                    println!("{}", result);
                }
            },
            Err(e) => {
                println!("❌ Error executing post-commit hook: {}", e);
            }
        }

        Ok(())
    }
}
