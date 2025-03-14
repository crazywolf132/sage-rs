use anyhow::{anyhow, Result};
use std::process::Command;

/// Stashes current changes
pub fn stash_changes() -> Result<()> {
    let result = Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-m")
        .arg("Auto-stashed by sage tool")
        .output()?;
    
    if result.status.success() {
        return Ok(());
    }
    
    return Err(anyhow!("Failed to stash changes. {}", String::from_utf8(result.stderr)?));
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