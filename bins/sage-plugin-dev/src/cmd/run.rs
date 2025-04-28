use anyhow::Result;
use clap::Args;
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct Run {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Arguments to pass to the plugin
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,

    /// Show detailed output from the plugin
    #[arg(short, long)]
    pub verbose: bool,

    /// Show debug information
    #[arg(short, long)]
    pub debug: bool,
}

impl Run {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Check if the plugin has the run function
        let functions = plugin_manager.get_plugin_functions(&plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&"run".to_string()) {
            println!("⚠️  Warning: Plugin does not support run function");
            return Ok(());
        }

        // Simulate a CLI command
        println!("Simulating CLI command: sage plugin {} {}",
            plugin_name,
            if self.args.is_empty() { "".to_string() } else { self.args.join(" ") }
        );

        match test_run(&mut plugin_manager, &plugin_name, &self.args, self.debug) {
            Ok(result) => {
                if result.contains("Success") {
                    println!("✅ Command executed successfully");

                    // Extract and display the actual message
                    if let Some(message_start) = result.find("message\":") {
                        let message = &result[message_start + 10..];
                        if let Some(end) = message.find("}") {
                            let clean_message = message[..end-1].trim_matches('"');
                            println!("\nOutput:");
                            println!("{}", clean_message);
                        } else {
                            println!("\nOutput:");
                            println!("{}", result);
                        }
                    } else if self.verbose {
                        println!("\nRaw output:");
                        println!("{}", result);
                    }
                } else {
                    println!("❌ Command execution failed");
                    println!("\nPlugin output:");
                    println!("{}", result);
                }
            },
            Err(e) => {
                println!("❌ Error executing command: {}", e);
            }
        }

        Ok(())
    }
}
