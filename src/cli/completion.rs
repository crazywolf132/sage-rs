use crate::cli::Run;
use anyhow::Result;
use clap::{Parser, CommandFactory};
use clap_complete::{generate, shells::Bash, shells::Zsh, shells::Fish};
use std::io;

#[derive(Parser, Debug)]
pub struct CompletionArgs {
    /// Shell to generate completions for (bash, zsh, fish)
    #[clap(value_enum)]
    pub shell: Shell,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Run for CompletionArgs {
    async fn run(&self) -> Result<()> {
        let mut cmd = crate::cli::Cmd::command();
        let mut stdout = io::stdout();

        match self.shell {
            Shell::Bash => {
                generate(Bash, &mut cmd, "sage", &mut stdout);
            }
            Shell::Zsh => {
                generate(Zsh, &mut cmd, "sage", &mut stdout);
            }
            Shell::Fish => {
                generate(Fish, &mut cmd, "sage", &mut stdout);
            }
        }

        Ok(())
    }
}

// Simplified value validation for branch names - used by the CLI argument parser only
pub mod value_completion {
    use crate::git::branch;
    
    pub fn branch_names(value: &str) -> Result<String, String> {
        // Try to get branch names
        match branch::list() {
            Ok(branches) => {
                // If value is empty or matches beginning of a branch, it's valid
                if value.is_empty() || branches.iter().any(|b| b.starts_with(value)) {
                    return Ok(value.to_string());
                }
                
                // If it doesn't match any branch prefix, but it's explicitly provided, accept it
                // (let users create new branches by explicitly typing them)
                if !value.is_empty() {
                    return Ok(value.to_string());
                }
                
                // For better error messages
                Err(format!("Invalid branch name: {}. Available branches: {}", 
                            value, 
                            branches.join(", ")))
            },
            Err(_) => {
                // If we couldn't get branch list, just accept the input
                // (This happens outside a git repo)
                Ok(value.to_string())
            }
        }
    }
} 