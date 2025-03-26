use anyhow::Result;
use inquire::Select;

use crate::git;

/// Displays an interactive branch selector and returns the selected branch name
pub fn select_branch() -> Result<String> {
    // Get all branches with their info
    let branches = git::branch::list_with_info()?;
    
    // Create display strings for each branch
    let branch_displays: Vec<String> = branches
        .iter()
        .map(|b| {
            let current_marker = if b.is_current { "* " } else { "  " };
            let tracking_info = match &b.upstream {
                Some(upstream) => {
                    let ahead_behind = match (b.ahead_count, b.behind_count) {
                        (0, 0) => String::new(),
                        (ahead, 0) => format!(" ↑{}", ahead),
                        (0, behind) => format!(" ↓{}", behind),
                        (ahead, behind) => format!(" ↑{}↓{}", ahead, behind),
                    };
                    format!(" → {}{}", upstream, ahead_behind)
                }
                None => String::new(),
            };
            format!("{}{}{}", current_marker, b.name, tracking_info)
        })
        .collect();

    // Create a mapping of display strings to branch names
    let branch_map: Vec<(String, String)> = branches
        .iter()
        .zip(branch_displays.iter())
        .map(|(branch, display)| (display.clone(), branch.name.clone()))
        .collect();

    // Show the selector
    let selection = Select::new("Select a branch to switch to:", branch_displays)
        .with_help_message("↑↓ to move, enter to select, esc to cancel")
        .prompt()?;

    // Find the corresponding branch name for the selected display string
    let selected_branch = branch_map
        .into_iter()
        .find(|(display, _)| display == &selection)
        .map(|(_, name)| name)
        .ok_or_else(|| anyhow::anyhow!("Failed to map selection to branch name"))?;

    Ok(selected_branch)
} 