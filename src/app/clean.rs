use anyhow::Result;
use octocrab::models::IssueState;
use crate::{git, errors, gh::pulls};
use colored::Colorize;

pub async fn clean() -> Result<()> {
    // Check to ensure we are in a repo first.
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotARepository.into());
    }

    let cleanable_branches = find_cleanable_branches().await?;
    
    if cleanable_branches.is_empty() {
        println!("No branches to clean! Everything is tidy.");
        return Ok(());
    }

    println!("\nThe following branches can be cleaned:");
    for branch in &cleanable_branches {
        println!("  {}", branch.blue());
    }

    // Ask for confirmation
    println!("\nDo you want to delete these branches? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Delete the branches
    for branch in cleanable_branches {
        // Try to delete remote first if it exists
        if git::branch::exists(&format!("origin/{}", branch)) {
            if let Err(e) = git::branch::delete_remote(&branch) {
                println!("{} Failed to delete remote branch '{}': {}", "WARNING:".yellow(), branch, e);
            } else {
                println!("Deleted remote branch: {}", branch.blue());
            }
        }

        // Then delete local
        if git::branch::exists(&branch) {
            if let Err(e) = git::branch::delete_local(&branch) {
                println!("{} Failed to delete local branch '{}': {}", "WARNING:".yellow(), branch, e);
            } else {
                println!("Deleted local branch: {}", branch.blue());
            }
        }
    }

    Ok(())
}

// Core logic for determining if a branch should be cleaned
fn should_clean_branch(
    branch_info: &git::branch::BranchInfo,
    current_branch: &str,
    default_branch: &str,
    merged_branches: &[String],
    pr_state: Option<&IssueState>,
    pr_merged: bool,
    upstream_exists: bool,
) -> bool {
    // Never clean current or default branch
    if branch_info.name == current_branch || branch_info.name == default_branch {
        return false;
    }

    // Clean if branch is merged
    if merged_branches.contains(&branch_info.name) {
        return true;
    }

    // Clean if PR is closed or merged
    if let Some(state) = pr_state {
        if *state == IssueState::Closed || pr_merged {
            return true;
        }
    }

    // Clean if upstream is configured but doesn't exist
    if let Some(_) = &branch_info.upstream {
        if !upstream_exists {
            return true;
        }
    }

    false
}

async fn find_cleanable_branches() -> Result<Vec<String>> {
    // Getting the latest remote.
    git::repo::fetch_remote()?;

    // Get the default branch and current branch
    let default_branch = git::repo::default_branch()?;
    let current_branch = git::branch::current()?;

    println!("Current branch: {}", current_branch);
    println!("Default branch: {}", default_branch);

    // Get detailed branch information including tracking info
    let branch_infos = git::branch::list_with_info()?;
    let merged_branches: Vec<String> = git::list::merged()?
        .into_iter()
        .filter(|branch| *branch != default_branch && *branch != current_branch)
        .collect();

    let mut cleanable_branches = Vec::new();

    // Process each local branch
    for branch_info in branch_infos {
        let branch_name = &branch_info.name;

        // Get PR state if it exists
        let (pr_state, pr_merged) = if let Ok(Some(pr)) = pulls::get_by_branch(branch_name).await {
            (pr.state.clone(), pr.merged_at.is_some())
        } else {
            (None, false)
        };

        // Check if upstream exists (if branch has one)
        let upstream_exists = if let Some(upstream) = &branch_info.upstream {
            git::branch::exists(upstream)
        } else {
            false
        };

        if should_clean_branch(
            &branch_info,
            &current_branch,
            &default_branch,
            &merged_branches,
            pr_state.as_ref(),
            pr_merged,
            upstream_exists,
        ) {
            cleanable_branches.push(branch_name.clone());
        }
    }

    Ok(cleanable_branches)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test branch info
    fn create_branch_info(name: &str, upstream: Option<&str>, is_current: bool) -> git::branch::BranchInfo {
        git::branch::BranchInfo {
            name: name.to_string(),
            upstream: upstream.map(|s| s.to_string()),
            ahead_count: 0,
            behind_count: 0,
            is_current,
        }
    }

    #[test]
    fn test_should_clean_merged_branch() {
        let branch_name = "feature/test";
        let branch_info = create_branch_info(branch_name, Some("origin/feature/test"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "current",
            "main",
            &vec![branch_name.to_string()],
            None,
            false,
            true,
        );

        assert!(result, "Should clean merged branches");
    }

    #[test]
    fn test_should_not_clean_current_branch() {
        let current = "feature/current";
        let branch_info = create_branch_info(current, Some("origin/feature/current"), true);
        
        let result = should_clean_branch(
            &branch_info,
            current,
            "main",
            &vec![],
            None,
            false,
            true,
        );

        assert!(!result, "Should not clean current branch even if other conditions match");
    }

    #[test]
    fn test_should_not_clean_default_branch() {
        let default = "main";
        let branch_info = create_branch_info(default, Some("origin/main"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "feature/current",
            default,
            &vec![default.to_string()],
            None,
            false,
            true,
        );

        assert!(!result, "Should not clean default branch even if other conditions match");
    }

    #[test]
    fn test_should_clean_branch_with_deleted_remote() {
        let branch_info = create_branch_info("feature/deleted", Some("origin/feature/deleted"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "current",
            "main",
            &vec![],
            None,
            false,
            false, // upstream doesn't exist
        );

        assert!(result, "Should clean branch with deleted remote");
    }

    #[test]
    fn test_should_clean_branch_with_closed_pr() {
        let branch_info = create_branch_info("feature/closed-pr", Some("origin/feature/closed-pr"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "current",
            "main",
            &vec![],
            Some(&IssueState::Closed),
            false,
            true,
        );

        assert!(result, "Should clean branch with closed PR");
    }

    #[test]
    fn test_should_clean_branch_with_merged_pr() {
        let branch_info = create_branch_info("feature/merged-pr", Some("origin/feature/merged-pr"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "current",
            "main",
            &vec![],
            Some(&IssueState::Open), // PR state doesn't matter if merged
            true, // PR is merged
            true,
        );

        assert!(result, "Should clean branch with merged PR");
    }

    #[test]
    fn test_should_not_clean_active_branch() {
        let branch_info = create_branch_info("feature/active", Some("origin/feature/active"), false);
        
        let result = should_clean_branch(
            &branch_info,
            "current",
            "main",
            &vec![],
            Some(&IssueState::Open),
            false,
            true,
        );

        assert!(!result, "Should not clean active branch with open PR");
    }
}