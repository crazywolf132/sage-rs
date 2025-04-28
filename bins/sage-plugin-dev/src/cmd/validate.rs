use anyhow::Result;
use clap::Args;
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct Validate {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Show detailed validation information
    #[arg(short, long)]
    pub verbose: bool,
}

impl Validate {
    pub fn run(&self) -> Result<()> {
        let path = Path::new(&self.path);

        println!("Validating plugin: {}", path.display());

        // First validate the basic structure
        match validate_plugin(path) {
            Ok(()) => {
                println!("✅ Plugin structure validation passed");
            },
            Err(e) => {
                println!("❌ Plugin structure validation failed: {}", e);
                return Ok(());
            }
        }

        // Now try to load the plugin to validate it can be loaded
        match load_plugin(path) {
            Ok((plugin_manager, plugin_name)) => {
                println!("✅ Plugin loaded successfully");

                // Get plugin info
                match plugin_manager.get_plugin_info(&plugin_name) {
                    Some(info) => {
                        println!("✅ Plugin metadata validation passed");

                        // Check functions
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

                        for function in functions {
                            match function.as_str() {
                                "pre_push" => {
                                    has_pre_push = true;
                                    println!("    ✅ pre_push");
                                },
                                "post_commit" => {
                                    has_post_commit = true;
                                    println!("    ✅ post_commit");
                                },
                                "run" => {
                                    has_run = true;
                                    println!("    ✅ run");
                                },
                                _ => println!("    ❓ {}", function),
                            }
                        }

                        if !has_pre_push {
                            println!("    ❌ pre_push (missing)");
                        }
                        if !has_post_commit {
                            println!("    ❌ post_commit (missing)");
                        }
                        if !has_run {
                            println!("    ❌ run (missing)");
                        }

                        println!("\nValidation Summary:");
                        println!("  ✅ Plugin structure is valid");
                        println!("  ✅ Plugin can be loaded");
                        println!("  ✅ Plugin metadata is valid");

                        if has_pre_push && has_post_commit && has_run {
                            println!("  ✅ Plugin implements all standard functions");
                        } else {
                            println!("  ⚠️  Plugin is missing some standard functions");
                        }

                        println!("\nTest Commands:");
                        if has_pre_push {
                            println!("  sage-plugin-dev pre-push {}", path.display());
                        }
                        if has_post_commit {
                            println!("  sage-plugin-dev post-commit {}", path.display());
                        }
                        if has_run {
                            println!("  sage-plugin-dev run {}", path.display());
                        }
                    },
                    None => {
                        println!("❌ Failed to get plugin metadata");
                    }
                }
            },
            Err(e) => {
                println!("❌ Plugin loading failed: {}", e);
                println!("The plugin structure is valid, but it cannot be loaded.");
            }
        }

        Ok(())
    }
}
