//! Unified error type for the plugin host.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("extism: {0}")]
    Extism(#[from] extism::Error),

    #[error("serde: {0}")]
    Json(#[from] serde_json::Error),

    #[error("plugin returned error: {0}")]
    Plugin(String),

    #[error("manifest missing field: {0}")]
    Manifest(&'static str),
}

pub type Result<T> = std::result::Result<T, PluginError>;