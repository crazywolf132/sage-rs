use anyhow::Result;
use clap::Args;
use crate::util::clean_plugins_directory;

#[derive(Args)]
pub struct Clean {
    /// Perform the cleanup without confirmation
    #[arg(short, long)]
    force: bool,
}

impl Clean {
    pub fn run(&self) -> Result<()> {
        println!("Cleaning up the plugins directory...");
        
        if !self.force {
            println!("This will remove any files in the plugins directory that are not valid plugins or manifests.");
            println!("Are you sure you want to continue? (y/N)");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Cleanup cancelled.");
                return Ok(());
            }
        }
        
        match clean_plugins_directory() {
            Ok(removed_files) => {
                if removed_files.is_empty() {
                    println!("No files were removed. The plugins directory is already clean.");
                } else {
                    println!("Removed {} files:", removed_files.len());
                    for file in removed_files {
                        println!("  - {}", file);
                    }
                    println!("Plugins directory cleaned successfully.");
                }
            },
            Err(e) => {
                println!("Error cleaning plugins directory: {}", e);
            }
        }
        
        Ok(())
    }
}
