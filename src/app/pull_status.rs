use anyhow::{anyhow, Result};
use crate::{errors, gh::pulls, git, ui::ColorizeExt};
use colored::Colorize;

pub async fn pull_status(pr_number: Option<u64>) -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo().unwrap() {
        return Err(errors::GitError::NotARepository.into());
    }

    // We are here, so obviously we are within a repo.
    // We will get the repo and owner info from the remote URL
    let (owner, repo_name) = git::repo::owner_repo()?;

    // We are now going to get the proper PR number, as we can utilise the GitHub API to do this
    // and see if the current branch has a valid PR associated with it

    let cleaned_pr_number = match &pr_number {
        Some(pr_number) => *pr_number, // Dereference to get the value
        None => {
            // We will now get the current branch name
            let current_branch = git::branch::current()?;
            // We will now use the GitHub API to get the PR number associated with the current branch
            match pulls::get_pr_number(&owner, &repo_name, &current_branch).await? {
                Some(number) => number, // Use the value directly
                None => {
                    // If we can't find a PR associated with the current branch, we will return an error
                    return Err(anyhow!("No pull request associated with the current branch"));
                }
            }
        }
    };

    let pull_request = pulls::get_pull_request(&owner, &repo_name, cleaned_pr_number).await?;

    println!("{} #{}: {}", "Pull Request".sage(), cleaned_pr_number, pull_request.title.unwrap().to_string().bright_white().bold());
    println!("{}", &pull_request.html_url.unwrap().to_string().url());
    println!();
    println!("Status: {}", format!("{:?}", pull_request.state.unwrap()).sage());
    println!("Branch: {} → {}", pull_request.head.ref_field.to_string().yellow().bold(), pull_request.base.ref_field.to_string().yellow().bold());
    println!();
    println!("{}", "Description:".sage()); 
    println!("{}", pull_request.body.unwrap_or("No description provided".to_string()));
    println!();

    // Get check runs for the PR
    let checks_response = pulls::get_checks(&owner, &repo_name, cleaned_pr_number).await?;
    
    // Display CI checks if they exist
    if let Some(total_count) = checks_response["total_count"].as_u64() {
        if total_count > 0 {
            println!("{}", "CI Checks:".sage());
            
            // Process the check runs array
            if let Some(check_runs) = checks_response["check_runs"].as_array() {
                for check in check_runs {
                    let name = check["name"].as_str().unwrap_or("Unknown check");
                    let status = check["status"].as_str().unwrap_or("unknown");
                    let conclusion = check["conclusion"].as_str();
                    
                    // Format the check status with color based on conclusion
                    let status_display = match conclusion {
                        Some("success") => format!("{}", "✓".green()),
                        Some("failure") => format!("{}", "✗".red()),
                        Some("cancelled") => format!("{}", "○".yellow()),
                        Some("skipped") => format!("{}", "-".bright_black()),
                        Some(other) => format!("{}", other.yellow()),
                        None => {
                            if status == "completed" {
                                format!("{}", "?".yellow())
                            } else {
                                format!("{}", "…".bright_black())
                            }
                        }
                    };
                    
                    println!("  {} {}", status_display, name);
                }
            }
            println!();
        }
    }
    
    if let Some(commits) = pull_request.commits {
        if commits > 0 {
            println!("{}", "Recent commits:".sage());
            for commit in pulls::get_timeline(&owner, &repo_name, cleaned_pr_number).await? {
                // Get the first 7 characters of the commit SHA
                let short_sha = &commit.sha[0..7];
                
                // Get the author login if available
                let author = commit.author.as_ref().map_or("unknown", |a| a.login.as_str());
                
                // Print commit info with colored components
                println!("  {}: {} by @{}", 
                         ColorizeExt::blue(short_sha), 
                         &commit.commit.message, 
                         author.to_string().yellow());
            }
        }
    }

    Ok(())
}