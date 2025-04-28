//! High-level helpers that *compose* core services for typical stacked-diff
//! commands. Think of this as the "domain facade" exposed to the CLI layer.
//! 
//! * Depends only on `sage-core`
//! * Provides convenience wrappers like `StackFacade::start_feature` that
//!   bundle multiple core service calls into one ergonomic API.

mod facade;
mod options;

pub use facade::StackFacade;
pub use options::{CreateChildOpts, RestackOpts};