use anyhow::{anyhow, Context, Result};
use auth_git2::GitAuthenticator;
use git2::{BranchType, Repository};
use std::process::Command;

/// current_branch returns the current branch name
pub fn current() -> Result<String> {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    let branch_name = String::from_utf8(result?.stdout)?;

    Ok(branch_name.trim().to_string())
}

/// switch_new switches to a branch, and will create it if required
pub fn switch_new(branch_name: &str, create: bool) -> Result<()> {
    let repo = Repository::open_from_env().context("Failed to open repository")?;

    if create {
        // Create new branch from HEAD commit.
        let head = repo.head().context("Failed to get HEAD reference")?;
        let commit = head
            .peel_to_commit()
            .context("Failed to convert HEAD to commit")?;
        repo.branch(&branch_name, &commit, false)
            .context("Failed to create new branch")?;
    }

    // Check if the branch exists locally
    let branch_ref = format!("refs/heads/{}", branch_name);
    if repo.find_reference(&branch_ref).is_err() {
        return Err(anyhow::anyhow!(
            "Branch '{}' does not exist and create=false",
            branch_name
        ));
    }

    // Set HEAD to the branch.
    repo.set_head(&branch_ref)
        .context("Failed to set HEAD to branch")?;

    // Checkout the branch with a clean forced checkout to ensure we get all changes
    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force(); // Force checkout to ensure we get all changes

    // Checkout the branch.
    repo.checkout_head(Some(&mut checkout_opts))
        .context("Failed to checkout branch")?;

    Ok(())
}

/// switch switches a branch, and will create it if required -- Returns current branch name
pub fn switch(branch_name: &str, create: bool) -> Result<String> {
    let current_branch = current()?;
    let mut cmd = Command::new("git");
    cmd.arg("switch");
    if create {
        cmd.arg("-c");
    }

    // This captures the output instead of displaying it
    let output = cmd
        .arg(branch_name)
        .output()
        .expect("failed to switch branch");

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to switch branch: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(current_branch)
}

/// list -- returns a list of the branches locally
pub fn list() -> Result<Vec<String>> {
    let repo = Repository::open_from_env().context("Failed to open repository")?;
    let mut branch_infos: Vec<(String, i64)> = Vec::new();
    let branches = repo
        .branches(Some(BranchType::Local))
        .context("Failed to get local branches")?;

    for branch in branches {
        let (branch, _) = branch?;
        let branch_name = branch
            .name()? // Get branch name as an Option<&str>
            .ok_or_else(|| anyhow!("Invalid UTF-8 in branch name"))?
            .to_string();

        // Retrieve the commit that the branch points to
        let branch_ref = branch.get();
        let commit_id = branch_ref
            .target()
            .ok_or_else(|| anyhow!("Branch has no target commit"))?;
        let commit = repo
            .find_commit(commit_id)
            .context("Failed to find commit for branch")?;
        let committer_date = commit.committer().when().seconds();

        branch_infos.push((branch_name, committer_date));
    }

    // Sort branches by descending committer date
    branch_infos.sort_by(|a, b| b.1.cmp(&a.1));

    // Extract branch names in sorted order
    let branch_names = branch_infos.into_iter().map(|(name, _)| name).collect();
    Ok(branch_names)
}

/// Get a struct containing information about a branch including its upstream, ahead and behind counts
#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub upstream: Option<String>,
    pub ahead_count: usize,
    pub behind_count: usize,
    pub is_current: bool,
}

/// list_with_info -- returns a list of branches with additional information
pub fn list_with_info() -> Result<Vec<BranchInfo>> {
    // Get the current branch first
    let current_branch = current()?;

    // Get all branches
    let branches = list()?;

    // Create a result vector
    let mut result = Vec::with_capacity(branches.len());

    for branch in branches {
        let (upstream, ahead, behind) = get_branch_tracking_info(&branch)?;

        result.push(BranchInfo {
            name: branch.clone(),
            upstream,
            ahead_count: ahead,
            behind_count: behind,
            is_current: branch == current_branch,
        });
    }

    Ok(result)
}

/// Get tracking information for a specific branch
/// Returns a tuple of (upstream_branch, ahead_count, behind_count)
fn get_branch_tracking_info(branch: &str) -> Result<(Option<String>, usize, usize)> {
    // Get the upstream branch
    let upstream_output = Command::new("git")
        .args([
            "for-each-ref",
            "--format=%(upstream:short)",
            &format!("refs/heads/{}", branch),
        ])
        .output()
        .context("Failed to get upstream branch")?;

    let upstream_str = String::from_utf8(upstream_output.stdout)?
        .trim()
        .to_string();

    // If there's no upstream branch, return early
    if upstream_str.is_empty() {
        return Ok((None, 0, 0));
    }

    // Now get ahead/behind counts
    let rev_list_args = format!("{}...{}", upstream_str, branch);
    let count_output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", &rev_list_args])
        .output()
        .context("Failed to get ahead/behind counts")?;

    if !count_output.status.success() {
        return Ok((Some(upstream_str), 0, 0));
    }

    // Parse the output
    let counts = String::from_utf8(count_output.stdout)?.trim().to_string();

    let parts: Vec<&str> = counts.split_whitespace().collect();
    let behind = if parts.len() > 0 {
        parts[0].parse().unwrap_or(0)
    } else {
        0
    };
    let ahead = if parts.len() > 1 {
        parts[1].parse().unwrap_or(0)
    } else {
        0
    };

    Ok((Some(upstream_str), ahead, behind))
}

/// push will push the current branch to remote
pub fn push(branch_name: &str, force: bool) -> Result<()> {
    // Create a git push command
    let mut cmd = Command::new("git");
    cmd.arg("push")
       .arg("--set-upstream")
       .arg("origin")
       .arg(branch_name);
    
    // Add force options based on the force parameter
    if force {
        cmd.arg("--force");
    } else {
        cmd.arg("--force-with-lease");
    }
    
    // Execute the command
    let result = cmd.output()?;
    
    if result.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to push branch: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}

/// exists returns if a branch exists
pub fn exists(branch_name: &str) -> bool {
    let branches = list().unwrap_or(vec![]);
    branches.iter().any(|b| b == branch_name)
}

/// set_upstream with a specific refspec
pub fn set_upstream(refspec: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("branch")
        .arg("--set-upstream-to")
        .arg(format!("origin/{}", refspec))
        .output()?;

    if result.status.success() {
        return Ok(());
    }
    Ok(())
}

/// merge will merge a specific branch into the current branch
pub fn merge(branch_name: &str) -> Result<()> {
    let result = Command::new("git").arg("merge").arg(branch_name).output()?;

    if result.status.success() {
        return Ok(());
    }

    Err(anyhow!(
        "Failed to merge branch: {}",
        String::from_utf8_lossy(&result.stderr)
    ))
}

/// rebase will rebase a specific branch onto the current branch
pub fn rebase(branch_name: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("rebase")
        .arg(branch_name)
        .arg("--autostash")
        .output()?;

    if result.status.success() {
        return Ok(());
    }

    Err(anyhow!(
        "Failed to rebase branch: {}",
        String::from_utf8_lossy(&result.stderr)
    ))
}

/// List conflicting files within the branch
pub fn conflicting_files() -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("--diff-filter=U")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list conflicting files: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }

    let output_str = String::from_utf8(output.stdout)?;
    let files: Vec<&str> = output_str.split_whitespace().collect();
    Ok(files.iter().map(|f| f.to_string()).collect())
}

/// Delete a local branch
pub fn delete_local(branch_name: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("branch")
        .arg("-D")  // Force delete
        .arg(branch_name)
        .output()?;

    if result.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to delete local branch: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}

/// Delete a remote branch
pub fn delete_remote(branch_name: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("--delete")
        .arg(branch_name)
        .output()?;

    if result.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to delete remote branch: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}
