use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents the result of a Git hook execution.
pub struct HookResult {
    /// The name of the hook (e.g., "pre-commit").
    pub name: String,
    /// Whether the hook passed (exit code 0).
    pub passed: bool,
    /// Combined stdout and stderr output from the hook.
    pub output: String,
}

/// Returns the path to the Git directory (e.g., ".git").
fn get_git_dir() -> Result<PathBuf> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("Failed to get git dir"));
    }
    let dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(dir))
}

/// Returns the path to the hooks directory within the Git directory.
fn get_hooks_dir() -> Result<PathBuf> {
    let git_dir = get_git_dir()?;
    Ok(git_dir.join("hooks"))
}

/// Checks whether the given path is executable.
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            let perms = meta.permissions();
            perms.mode() & 0o111 != 0
        } else {
            false
        }
    }
    #[cfg(windows)]
    {
        // On Windows, assume the hook script can execute if it exists.
        true
    }
}

/// Runs the commit-related Git hooks (pre-commit and commit-msg) manually.
/// It returns a vector of HookResult in the order run.
pub fn run_commit_hooks(message: &str) -> Result<Vec<HookResult>> {
    let hooks_dir = get_hooks_dir()?;
    let mut results = Vec::new();

    // Define the commit hooks to run, in order.
    let hook_names = ["pre-commit", "commit-msg"];
    // Path to temporary file for commit message (for commit-msg hook).
    let mut commit_msg_file_path: Option<PathBuf> = None;

    for hook_name in &hook_names {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() && hook_path.is_file() && is_executable(&hook_path) {
            // Determine arguments for the hook.
            let args = if *hook_name == "commit-msg" {
                // Prepare a temporary file containing the commit message.
                if commit_msg_file_path.is_none() {
                    let mut tmp = env::temp_dir();
                    let file_name = format!("sage_commit_msg_{}.txt", std::process::id());
                    tmp.push(&file_name);
                    fs::write(&tmp, message)?;
                    commit_msg_file_path = Some(tmp.clone());
                }
                vec![commit_msg_file_path.as_ref().unwrap().to_string_lossy().to_string()]
            } else {
                Vec::new()
            };
            // Execute the hook script.
            let mut cmd = Command::new(&hook_path);
            cmd.args(&args);
            let output = cmd.output()?;
            let passed = output.status.success();
            // Combine stdout and stderr for display.
            let mut out = String::new();
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stdout.trim().is_empty() {
                out.push_str(&stdout);
            }
            if !stderr.trim().is_empty() {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&stderr);
            }
            results.push(HookResult {
                name: hook_name.to_string(),
                passed,
                output: out,
            });
            // Stop further hooks if one failed.
            if !passed {
                break;
            }
        }
    }

    // Clean up the temporary commit message file if it was created.
    if let Some(path) = commit_msg_file_path {
        let _ = fs::remove_file(path);
    }

    Ok(results)
}