use anyhow::{anyhow, Result};
use colored::Colorize;
use crate::git::hooks;
use std::process::Command;

/// list_branches returns a list of all local branches
pub fn list_branches() -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)")
        .output()
        .expect("Could not list commits");

    let stdout = String::from_utf8_lossy(&result.stdout);
    let lines = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    Ok(lines)
}

/// is_repo returns true or false if the current dir is a git repo
pub fn is_repo() -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Could not get commit hash")
        .stdout;

    let out = String::from_utf8_lossy(&output);
    Ok(out.eq("true"))
}

/// current_branch returns the name of the current branch
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Could not get commit hash");

    let out = String::from_utf8_lossy(&output.stdout);
    Ok(out.trim().to_string())
}

/// is_clean will return a bool based on if the current repo state is clean
pub fn is_clean() -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Could not get commit hash");

    let out = String::from_utf8_lossy(&output.stdout);
    Ok(out.trim().eq(""))
}

/// commit creates a new commit with message
/// skip_hooks will bypass manual hook execution if true
pub fn commit(message: &str, empty: bool, skip_hooks: bool) -> Result<()> {
    // Run commit hooks manually (pre-commit and commit-msg) unless skipped
    if !skip_hooks {
        let hook_results = hooks::run_commit_hooks(message)?;
        // Summarise hook results
        for hr in &hook_results {
            if hr.passed {
                println!("{} {}", "✓".green(), hr.name);
            } else {
                // On failure, display full hook output and abort
                if !hr.output.trim().is_empty() {
                    println!("{}", hr.output);
                }
                return Err(anyhow!(format!("Hook {} failed", hr.name)));
            }
        }
    }

    // Create the commit with no-verify to skip built-in hook execution
    let mut cmd = Command::new("git");
    cmd.arg("commit")
        .arg("--no-verify")
        .arg("-m")
        .arg(message);
    if empty {
        cmd.arg("--allow-empty");
    }

    let res = cmd.output()?;
    if res.status.success() {
        // Display git commit output if any
        let stdout = String::from_utf8_lossy(&res.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout.trim());
        }
        return Ok(());
    }

    // On failure, display error output
    let stderr = String::from_utf8_lossy(&res.stderr);
    Err(anyhow!(format!("failed to create commit message: {}", stderr.trim())))
}

/// Create a temporary WIP commit with all current changes
pub fn create_wip_commit() -> Result<()> {
    // First add all changes
    let add = Command::new("git")
        .args(["add", "."])
        .output()?;

    if !add.status.success() {
        return Err(anyhow!("Failed to stage changes for WIP commit"));
    }

    // Create the WIP commit
    let commit = Command::new("git")
        .args(["commit", "-m", "[SAGE WIP] Temporary commit for sync"])
        .output()?;

    if !commit.status.success() {
        return Err(anyhow!("Failed to create WIP commit"));
    }

    Ok(())
}

/// Pop the most recent WIP commit but keep the changes
pub fn pop_wip_commit() -> Result<()> {
    // Reset the WIP commit but keep changes
    let reset = Command::new("git")
        .args(["reset", "--soft", "HEAD~1"])
        .output()?;

    if !reset.status.success() {
        return Err(anyhow!("Failed to pop WIP commit"));
    }

    Ok(())
}
