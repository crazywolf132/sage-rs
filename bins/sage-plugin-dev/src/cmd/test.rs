use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::Path;
use crate::util::*;

#[derive(Args)]
pub struct Test {
    #[command(subcommand)]
    command: TestCommand,
}

#[derive(Subcommand)]
enum TestCommand {
    /// Test the pre-push hook
    PrePush {
        /// Path to the plugin file (.wasm or .js)
        path: String,

        /// Branch name to use in the test
        #[arg(default_value = "main")]
        branch: String,
    },

    /// Test the post-commit hook
    PostCommit {
        /// Path to the plugin file (.wasm or .js)
        path: String,

        /// Commit ID to use in the test
        #[arg(default_value = "abcdef1234567890")]
        commit_id: String,
    },

    /// Test the CLI command
    Run {
        /// Path to the plugin file (.wasm or .js)
        path: String,

        /// Arguments to pass to the plugin
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

impl Test {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            TestCommand::PrePush { path, branch } => {
                let (mut plugin_manager, plugin_name) = load_plugin(Path::new(path))?;

                println!("Testing pre-push hook for plugin: {}", plugin_name);
                println!("Branch: {}", branch);

                match test_pre_push(&mut plugin_manager, &plugin_name, branch) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
            },

            TestCommand::PostCommit { path, commit_id } => {
                let (mut plugin_manager, plugin_name) = load_plugin(Path::new(path))?;

                println!("Testing post-commit hook for plugin: {}", plugin_name);
                println!("Commit ID: {}", commit_id);

                match test_post_commit(&mut plugin_manager, &plugin_name, commit_id) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
            },

            TestCommand::Run { path, args } => {
                let (mut plugin_manager, plugin_name) = load_plugin(Path::new(path))?;

                println!("Testing CLI command for plugin: {}", plugin_name);
                println!("Args: {:?}", args);

                match test_run(&mut plugin_manager, &plugin_name, args) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
            },
        }

        Ok(())
    }
}
