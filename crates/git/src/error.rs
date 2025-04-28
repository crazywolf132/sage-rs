use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("git not installed")]
    NotInstalled,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("git exited with {status}: {stderr}")]
    Exit { status: i32, stderr: String },
    #[error("parse: {0}")]
    Parse(String),
    #[error("timeout waiting for git")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, GitError>;
