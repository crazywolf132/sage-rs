use anyhow::Result;
use clap::Args;
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct Info {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,
}

impl Info {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Get plugin info
        let info = get_plugin_info(&plugin_manager, &plugin_name)?;

        // Get functions
        let functions = match plugin_manager.get_plugin_functions(&plugin_name) {
            Some(funcs) => funcs.clone(),
            None => Vec::new(),
        };

        println!("\nPlugin Information:");
        println!("  Name:     {}", info.name);
        println!("  Version:  {}", info.version);
        println!("  Type:     {:?}", info.plugin_type);
        println!("  Functions:");

        let mut has_pre_push = false;
        let mut has_post_commit = false;
        let mut has_run = false;

        for function in &functions {
            match function.as_str() {
                "pre_push" => {
                    has_pre_push = true;
                    println!("    • pre_push - Git pre-push hook");
                },
                "post_commit" => {
                    has_post_commit = true;
                    println!("    • post_commit - Git post-commit hook");
                },
                "run" => {
                    has_run = true;
                    println!("    • run - CLI command");
                },
                _ => println!("    • {} - Custom function", function),
            }
        }

        println!("\nUsage:");
        if has_run {
            println!("  sage plugin {}", plugin_name);
            println!("  sage plugin {} [args...]", plugin_name);
        }

        println!("\nTest Commands:");
        if has_pre_push {
            println!("  sage-plugin-dev pre-push {} --branch=main", plugin_path.display());
        }
        if has_post_commit {
            println!("  sage-plugin-dev post-commit {} --commit-id=abc123", plugin_path.display());
        }
        if has_run {
            println!("  sage-plugin-dev run {} [args...]", plugin_path.display());
        }

        if self.verbose {
            println!("\nManifest Path:");
            println!("  {}", plugin_path.with_extension("json").display());

            // Try to read the manifest
            if let Ok(manifest_content) = std::fs::read_to_string(plugin_path.with_extension("json")) {
                println!("\nManifest Content:");
                println!("{}", manifest_content);
            }
        }

        Ok(())
    }
}
