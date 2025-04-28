use clap::{Args, Subcommand};
use anyhow::Result;
use crate::cmd::Runtime;

#[derive(Args)]
pub struct Plugin {
    #[command(subcommand)]
    command: Option<PluginCommand>,

    /// Plugin name to run (shorthand for `run <name>`)
    name: Option<String>,

    /// Arguments to pass to the plugin
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum PluginCommand {
    /// List all installed plugins
    List,
    
    /// Run a plugin
    Run {
        /// Name of the plugin to run
        name: String,
        
        /// Arguments to pass to the plugin
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    
    /// Install a plugin from a WASM file
    Install {
        /// Path to the WASM file
        path: String,
    },
}

impl Plugin {
    pub fn run(self, rt: &mut Runtime) -> Result<()> {
        match self.command {
            Some(PluginCommand::List) => {
                let plugins = rt.plugins.list_plugins()?;
                if plugins.is_empty() {
                    println!("No plugins installed.");
                    println!("Install plugins in ~/.config/sage/plugins/");
                    return Ok(());
                }
                
                println!("Installed plugins:");
                for plugin in plugins {
                    println!("  - {}", plugin);
                }
            }
            
            Some(PluginCommand::Run { name, args }) => {
                let output = rt.plugins.run_cli(&name, &args)?;
                print!("{}", output);
            }
            
            Some(PluginCommand::Install { path }) => {
                rt.plugins.install_plugin(&path)?;
                println!("Plugin installed successfully.");
            }
            
            None => {
                // Handle the shorthand case: `sage plugin <name> [args...]`
                if let Some(name) = self.name {
                    let output = rt.plugins.run_cli(&name, &self.args)?;
                    print!("{}", output);
                } else {
                    // No subcommand and no name, show help
                    println!("Usage: sage plugin <name> [args...]");
                    println!("       sage plugin list");
                    println!("       sage plugin run <name> [args...]");
                    println!("       sage plugin install <path>");
                }
            }
        }
        
        Ok(())
    }
}
