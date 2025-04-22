use anyhow::{anyhow, Result};
use std::process::Command;

/// Stashes current changes
pub fn stash_changes() -> Result<()> {
    let result = Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-m")
        .arg("Auto-stashed by sage")
        .output()?;
    
    if result.status.success() {
        return Ok(());
    }
    
    return Err(anyhow!("Failed to stash changes. {}", String::from_utf8(result.stderr)?));
}

/// Determines if there are any stashes
pub fn has_stash() -> Result<bool> {
    let result = Command::new("git")
        .arg("stash")
        .arg("list")
        .output()?;
    
    if result.status.success() {
        return Ok(true);
    }
    
    if let Ok(stderr) = String::from_utf8(result.stderr.clone()) {
        if stderr.contains("No stash entries found") {
            return Ok(false);
        }
    }
        
    Err(anyhow!("Failed to check for stashes. {}", String::from_utf8(result.stderr)?))
}

/// Applies and drops the most recent stash
pub fn apply_stash() -> Result<()> {
    let result = Command::new("git")
        .arg("stash")
        .arg("pop")
        .output()?;
    
    if result.status.success() {
        return Ok(());
    }
    
    // Even if applying the stash fails due to conflicts, we want to let the user handle it
    // rather than blocking the process entirely
    if let Ok(stderr) = String::from_utf8(result.stderr.clone()) {
        if stderr.contains("conflict") {
            println!("Note: There were conflicts when applying your changes. Please resolve them manually.");
            return Ok(());
        }
    }
    
    return Err(anyhow!("Failed to apply stashed changes. {}", String::from_utf8(result.stderr)?));
}

/// Pops the most recent stash
pub fn pop() -> Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .arg("pop")
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to pop stash: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// Pops the stash at the given reference (e.g., stash@{0})
pub fn pop_with_ref(stash_ref: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .arg("pop")
        .arg(stash_ref)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to pop stash {}: {}", stash_ref, String::from_utf8_lossy(&output.stderr)))
    }
}

/// Pops the most recent stash, or a specific stash ref if provided
pub fn pop_stash(stash_ref: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("stash").arg("pop");
    if let Some(s) = stash_ref {
        cmd.arg(s);
    }
    let output = cmd.output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to pop stash: {}", String::from_utf8_lossy(&output.stderr)))
    }
}