use thiserror::Error;

/// Error type for git operations
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),
    
    #[error("Git command not found")]
    GitNotFound,
    
    #[error("Not a git repository")]
    NotARepository,
    
    #[error("Invalid git output: {0}")]
    InvalidOutput(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
} 