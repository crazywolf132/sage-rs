use crate::{cli::Run, git};
use clap::Parser;

use anyhow::{Result, anyhow};
use colored::Colorize;
use std::path::Path;

#[derive(Parser, Debug)]
pub struct CloneArgs {
    /// Name of the repo to clone (format: owner/repo)
    name: String,
    
    /// Use SSH for cloning instead of HTTPS
    #[clap(long, short)]
    ssh: bool,
}

impl Run for CloneArgs {
    async fn run(&self) -> Result<()> {
        // Validate that the repo name has the correct format
        if !self.name.contains('/') {
            return Err(anyhow!("Please provide a repo name in the format: {}", "owner/repo".color("green")));
        }
        
        // Get the repo name from the path
        let repo_name = self.name.split('/').last()
            .ok_or_else(|| anyhow!("Invalid repository path format"))?;
            
        // Check if a directory with the repo name already exists
        if Path::new(repo_name).exists() {
            return Err(anyhow!("Directory '{}' already exists", repo_name));
        }

        // Clone the repo
        let protocol = if self.ssh { "SSH" } else { "HTTPS" };
        println!("Cloning {} from GitHub using {}...", self.name.color("yellow"), protocol);
        
        match git::repo::clone(&self.name, self.ssh) {
            Ok(_) => {
                println!("Successfully cloned: {}", repo_name.color("green"));
                Ok(())
            },
            Err(e) => {
                eprintln!("{}: {}", "Error".color("red"), e);
                Err(e)
            }
        }
    }
}
