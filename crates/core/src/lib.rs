//! Sage **core** - pure, deterministic business logic.
//!
//! * No I/O, no async, no git2.
//! * All side-effects expressed as `port::git::GitAction` or return values.
//! * External crates depend **on** core, never the other way.

pub mod error;
pub mod model;
pub mod port;
pub mod service;
pub mod util;

pub use error::CoreError;
