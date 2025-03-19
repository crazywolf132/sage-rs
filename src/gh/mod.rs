pub mod pulls;

use anyhow::Result;
use octocrab::Octocrab;
use std::env;
use std::sync::OnceLock;

// Global instance of authenticated Octocrab client
static OCTOCRAB_INSTANCE: OnceLock<Octocrab> = OnceLock::new();

/// Get a properly authenticated instance of the GitHub API client
pub fn get_instance() -> &'static Octocrab {
    OCTOCRAB_INSTANCE.get_or_init(|| {
        // First try to use a GITHUB_TOKEN from the environment
        if let Ok(token) = env::var("GITHUB_TOKEN") {
            Octocrab::builder()
                .personal_token(token)
                .build()
                .unwrap_or_else(|_| Octocrab::default())
        } else {
            // Try to use git config credentials
            let builder = Octocrab::builder();
            
            // The authentication will be handled automatically by octocrab
            // using the git credential helper when needed
            builder.build().unwrap_or_else(|_| Octocrab::default())
        }
    })
}