use crate::{git, ui::ColorizeExt};
use anyhow::Result;
use colored::Colorize;

/// history will show the history of commits
pub fn history() -> Result<()> {
    // Get the commits
    let commits = git::list::commits()?;
    let current_branch = git::branch::current()?;

    println!(
        "{} {}",
        "Branch History:".sage().bold(),
        current_branch.yellow()
    );
    if commits.is_empty() {
        println!("{}", "No commits found".bright_green());
        return Ok(());
    }

    // Group commits by date
    let mut current_date = String::new();

    for commit in &commits {
        // If we encounter a new date, print it
        if commit.date != current_date {
            current_date = commit.date.clone();
            println!();
            println!("{} {}", "Date:".bright_blue(), current_date.bold());
        }

        // Print commit info in the desired format
        println!(
            " {} {} {} @{}",
            "●".sage(),
            commit.hash.bright_yellow(),
            "by".gray(),
            commit.author
        );

        // Print the commit message indented
        if !commit.message.is_empty() {
            println!("   {}", commit.message);
        }
    }

    Ok(())
}
