//! # Sage Plugin API
//!
//! This crate provides a plugin system for Sage using WebAssembly (WASM) via Extism
//! and JavaScript plugins.
//!
//! ## Overview
//!
//! The plugin system allows extending Sage with custom functionality through plugins.
//! Plugins can:
//!
//! - Hook into git lifecycle events (pre-push, post-commit)
//! - Add custom CLI commands
//! - Integrate with any part of the application
//!
//! ## Plugin Types
//!
//! Sage supports two types of plugins:
//!
//! 1. **WASM Plugins**: Compiled WebAssembly modules
//! 2. **JavaScript Plugins**: JavaScript files that can be executed
//!
//! ## Plugin Structure
//!
//! Each plugin consists of two files:
//! - A `.wasm` file containing the compiled WebAssembly code or a `.js` file containing JavaScript code
//! - A `.json` manifest file with metadata and configuration
//!
//! ## Creating Plugins
//!
//! WASM plugins can be created in any language that compiles to WebAssembly, including:
//! - Rust (with `wasm-bindgen` or `wasm-pack`)
//! - AssemblyScript
//! - C/C++ (with Emscripten)
//! - Go (with TinyGo)
//!
//! JavaScript plugins can be created using standard JavaScript.
//!
//! ## Plugin Manifest
//!
//! The manifest file (`.json`) should contain:
//!
//! ```json
//! {
//!   "name": "my-plugin",
//!   "version": "1.0.0",
//!   "functions": ["pre_push", "post_commit", "run"]
//! }
//! ```
//!
//! ## Plugin Functions
//!
//! Plugins can export the following functions:
//!
//! - `pre_push`: Called before pushing to a remote
//! - `post_commit`: Called after a commit is created
//! - `run`: Called when the plugin is executed as a CLI command
//!
//! ## Using Plugins in Code
//!
//! ### Basic Usage with PluginRegistry
//!
//! The `PluginRegistry` provides a centralized way to access plugins from different parts of the code:
//!
//! ```rust
//! // Initialize the plugin registry
//! let plugin_manager = PluginManager::load_dir("~/.config/sage/plugins")?;
//! let registry = PluginRegistry::new(plugin_manager);
//!
//! // Run a plugin's CLI command
//! let output = registry.run_cli("my-plugin", &["arg1", "arg2"])?;
//!
//! // Run lifecycle hooks
//! registry.run_pre_push("main")?;
//! registry.run_post_commit("abc123");
//! ```
//!
//! ### Using GitHooks
//!
//! The `GitHooks` struct provides a convenient way to integrate plugins with git operations:
//!
//! ```rust
//! // Initialize the plugin registry
//! let plugin_manager = PluginManager::load_dir("~/.config/sage/plugins")?;
//! let registry = PluginRegistry::new(plugin_manager);
//!
//! // Create a GitHooks instance
//! let hooks = GitHooks::new(registry.clone());
//!
//! // Run hooks during git operations
//! hooks.pre_push("main")?;
//! hooks.post_commit("abc123");
//! ```
//!
//! ### Using the WithPlugins Trait
//!
//! The `WithPlugins` trait can be implemented by any struct that has a plugin registry:
//!
//! ```rust
//! struct MyGitService {
//!     // ... other fields
//!     registry: PluginRegistry,
//! }
//!
//! impl WithPlugins for MyGitService {
//!     fn plugin_registry(&self) -> &PluginRegistry {
//!         &self.registry
//!     }
//! }
//!
//! // Now you can use the trait methods
//! let service = MyGitService { /* ... */ };
//! service.run_pre_push("main")?;
//! service.run_post_commit("abc123");
//! ```

mod error;
mod host;
mod registry;
mod hooks;

pub use error::{PluginError, Result};
pub use host::{Event, PluginInfo, PluginManager, PluginType, Reply};
pub use registry::PluginRegistry;
pub use hooks::{GitHooks, WithPlugins};