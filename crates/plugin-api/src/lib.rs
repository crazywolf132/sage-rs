//! Public entry‑point: re‑export JSON schemas + PluginManager so the main Sage
//! crate only needs a single dependency.

mod error;
mod host;

pub use error::{PluginError, Result};
pub use host::{Event, PluginManager, Reply};