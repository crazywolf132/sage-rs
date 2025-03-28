use crate::{cli::Run, git};
use clap::Parser;

use anyhow::{Result, anyhow};
use colored::Colorize;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(after_help = "This command simplifies the process of cloning GitHub repositories by using a more intuitive syntax. \
Instead of typing the full GitHub URL, you can simply provide the repository in the format 'owner/repo'. \
The command will handle constructing the proper URL based on your preferred protocol (HTTPS or SSH).")]
pub struct CloneArgs {
    /// Name of the repo to clone (format: owner/repo)
    #[clap(long_help = "The repository to clone in the format 'owner/repo'. \
This is the GitHub username or organization name, followed by a slash, \
followed by the repository name. For example: 'octocat/Hello-World' or 'rust-lang/rust'. \
The command will automatically construct the proper GitHub URL from this information.")]
    name: String,
    
    /// Use SSH for cloning instead of HTTPS
    #[clap(long, short, long_help = "Use SSH protocol for cloning instead of HTTPS (default). \
When this flag is set, the command will use 'git@github.com:owner/repo.git' format \
instead of 'https://github.com/owner/repo'. \
SSH is preferred if you have SSH keys set up with GitHub and want to avoid \
entering your username and password for each operation.")]
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
