use anyhow::Result;
use crate::{errors, git};
use colored::Colorize;

pub fn list() -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    println!("Branches:");
    // Getting all the branches with detailed information
    let branches = git::branch::list_with_info()?;
    
    for branch in branches {
        let mut output = String::new();
        
        // Mark current branch with an asterisk
        if branch.is_current {
            output.push_str("* ");
        } else {
            output.push_str("  ");
        }
        
        // Add branch name
        output.push_str(&branch.name);
        
        // Add tracking information if available
        if let Some(upstream) = branch.upstream {
            output.push_str(&format!(" -> {}", upstream));
            
            // Add ahead/behind information with arrows
            if branch.ahead_count > 0 || branch.behind_count > 0 {
                let mut counts = Vec::new();
                
                if branch.ahead_count > 0 {
                    counts.push(format!("↑{}", branch.ahead_count));
                }
                
                if branch.behind_count > 0 {
                    counts.push(format!("↓{}", branch.behind_count));
                }
                
                if !counts.is_empty() {
                    output.push_str(&format!(" [{}]", counts.join(", ")));
                }
            }
        }
        
        // Colorize differently based on status
        if branch.is_current {
            println!("{}", output.green());
        } else if branch.ahead_count > 0 && branch.behind_count > 0 {
            // Diverged branches - yellow
            println!("{}", output.yellow());
        } else if branch.ahead_count > 0 {
            // Branches ahead - cyan
            println!("{}", output.cyan());
        } else if branch.behind_count > 0 {
            // Branches behind - magenta
            println!("{}", output.magenta());
        } else {
            // Regular branches - blue
            println!("{}", output.blue());
        }
    }

    Ok(())
}