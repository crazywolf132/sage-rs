//! Thin wrapper around `extism::Plugin` that
//! * loads a `<n>.wasm` + `<n>.json` pair
//! * keeps each plugin in memory for reuse (fast!)
//! * dispatches lifecycle hooks by calling exported functions
//!   (functions must accept/return JSON strings)

use std::{collections::HashMap, fs, path::Path};

use extism::{Function, Plugin};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

use crate::error::*;

// ───────────────────────────────── Manifest ─────────────

#[derive(Debug, Deserialize)]
struct Manifest {
    name:    String,
    version: String,
    #[serde(default)]
    functions: Vec<String>, // e.g. ["pre_push", "post_commit"]
}

// ───────────────────────────────── JSON Schemas ─────────

/// Event sent **to** plugins.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    PrePush { branch: String },
    PostCommit { oid: String },
}

/// Reply **from** plugins.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Reply {
    Ok   { message: String },
    Error{ message: String },
}

impl Reply {
    pub fn ok<M: Into<String>>(msg: M) -> Self    { Self::Ok   { message: msg.into() } }
    pub fn error<M: Into<String>>(msg: M) -> Self { Self::Error{ message: msg.into() } }
}

// ─────────────────────────── PluginManager ──────────────

pub struct PluginManager {
    plugins: HashMap<String, Plugin>, // keyed by plugin name
}

impl PluginManager {
    /// Scan `$SAGE_PLUGINS` dir and load every `<n>.wasm`+manifest.
    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let mut plugins = HashMap::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("wasm") {
                continue;
            }
            let _stem = path.file_stem().unwrap().to_string_lossy();
            let manifest_path = path.with_extension("json");
            let manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;

            // load wasm bytes and create Extism plugin (no host functions)
            let wasm = fs::read(&path)?;
            let plugin = Plugin::new(wasm, Vec::<Function>::new(), false)?;
            info!(name = %manifest.name, version = %manifest.version, "loaded plugin");
            plugins.insert(manifest.name, plugin);
        }
        Ok(Self { plugins })
    }

    /// Dispatch a PrePush hook; first plugin that returns Error blocks.
    pub fn pre_push(&mut self, branch: &str) -> Result<()> {
        let input = serde_json::to_vec(&Event::PrePush { branch: branch.into() })?;
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();

        for name in plugin_names {
            if let Some(p) = self.plugins.get_mut(&name) {
                // Call the plugin function
                if !p.function_exists("pre_push") {
                    continue; // Skip if function doesn't exist
                }

                let result = p.call::<&[u8], Vec<u8>>("pre_push", &input)?;
                if result.is_empty() {
                    continue; // Plugin opted to return nothing
                }

                // Parse the response
                let out: Reply = serde_json::from_slice(&result)?;
                match out {
                    Reply::Ok { message } => info!(plugin=%name, %message),
                    Reply::Error { message } => return Err(PluginError::Plugin(format!("{name}: {message}"))),
                }
            }
        }
        Ok(())
    }

    /// Dispatch post‑commit (errors ignored, logged).
    pub fn post_commit(&mut self, oid: &str) {
        let input = serde_json::to_vec(&Event::PostCommit { oid: oid.into() }).unwrap();
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();

        for name in plugin_names {
            if let Some(p) = self.plugins.get_mut(&name) {
                // Call the plugin function
                if !p.function_exists("post_commit") {
                    continue; // Skip if function doesn't exist
                }

                // Call and handle errors
                match p.call::<&[u8], Vec<u8>>("post_commit", &input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            // Try to parse the response
                            if let Ok(Reply::Error { message }) = serde_json::from_slice::<Reply>(&result) {
                                debug!(plugin=%name, %message, "post_commit error ignored");
                            }
                        }
                    },
                    Err(e) => debug!(plugin=%name, error=%e, "post_commit call failed"),
                }
            }
        }
    }

    /// Run CLI plugin: `sage <plugin> ...args` → returns stdout string.
    pub fn cli(&mut self, plugin: &str, args: &[String]) -> Result<String> {
        let p = self.plugins.get_mut(plugin).ok_or_else(|| PluginError::Manifest("plugin not found"))?;
        let input = serde_json::to_vec(&json!({ "args": args }))?;

        // Check if the function exists
        if !p.function_exists("run") {
            return Err(PluginError::Manifest("plugin doesn't export 'run'"));
        }

        // Call the plugin function
        let result = p.call::<&[u8], Vec<u8>>("run", &input)?;
        if result.is_empty() {
            return Err(PluginError::Manifest("plugin returned empty response"));
        }

        // Parse the response
        let reply: Reply = serde_json::from_slice(&result)?;
        match reply {
            Reply::Ok { message } => Ok(message),
            Reply::Error { message } => Err(PluginError::Plugin(message)),
        }
    }
}
