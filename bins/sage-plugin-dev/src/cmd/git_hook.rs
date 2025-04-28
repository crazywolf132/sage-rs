use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use std::process::Command;
use crate::util::*;
use sage_plugin_api::PluginManager;

#[derive(Args)]
pub struct GitHook {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Type of git hook to simulate (pre-push or post-commit)
    #[arg(short, long, default_value = "post-commit")]
    pub hook_type: String,

    /// Show detailed output from the plugin
    #[arg(short, long)]
    pub verbose: bool,

    /// Show debug information
    #[arg(short, long)]
    pub debug: bool,

    /// Use the current git repository for real data
    #[arg(short, long)]
    pub real_data: bool,
}

impl GitHook {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        match self.hook_type.as_str() {
            "pre-push" => self.simulate_pre_push(&mut plugin_manager, &plugin_name)?,
            "post-commit" => self.simulate_post_commit(&mut plugin_manager, &plugin_name)?,
            _ => return Err(anyhow!("Unsupported hook type: {}", self.hook_type)),
        }

        Ok(())
    }

    fn simulate_pre_push(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Check if the plugin has the pre_push function
        let functions = plugin_manager.get_plugin_functions(plugin_name)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&"pre_push".to_string()) {
            println!("⚠️  Warning: Plugin does not support pre_push function");
            return Ok(());
        }

        // Get the current branch if using real data
        let branch = if self.real_data {
            get_current_branch()?
        } else {
            "main".to_string()
        };

        // Simulate a pre-push event
        println!("Simulating pre-push event for branch: {}", branch);
        println!("Plugin: {}", plugin_name);

        match test_pre_push(plugin_manager, plugin_name, &branch, self.debug) {
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

    fn simulate_post_commit(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        // Check if the plugin has the post_commit function
        let functions = plugin_manager.get_plugin_functions(plugin_name)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&"post_commit".to_string()) {
            println!("⚠️  Warning: Plugin does not support post_commit function");
            return Ok(());
        }

        // Get the latest commit if using real data
        let commit_id = if self.real_data {
            get_latest_commit()?
        } else {
            "abcdef1234567890".to_string()
        };

        // Simulate a post-commit event
        println!("Simulating post-commit event for commit: {}", commit_id);
        println!("Plugin: {}", plugin_name);

        match test_post_commit(plugin_manager, plugin_name, &commit_id, self.debug) {
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

// Helper function to get the current branch
fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to get current branch"));
    }

    let branch = String::from_utf8(output.stdout)?;
    Ok(branch.trim().to_string())
}

// Helper function to get the latest commit
fn get_latest_commit() -> Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to get latest commit"));
    }

    let commit = String::from_utf8(output.stdout)?;
    Ok(commit.trim().to_string())
}
