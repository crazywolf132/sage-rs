use thiserror::Error;

pub mod git;

// Re-export error types for convenient access
pub use git::{GitError, GitHubError};

// Generic Error type for the application
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Git error: {0}")]
    Git(#[from] GitError),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("{0}")]
    Other(String),
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
} 