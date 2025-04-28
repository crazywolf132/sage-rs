//! Thin wrapper around `extism::Plugin` that
//! * loads a `<n>.wasm` + `<n>.json` pair
//! * keeps each plugin in memory for reuse (fast!)
//! * dispatches lifecycle hooks by calling exported functions
//!   (functions must accept/return JSON strings)

use std::{collections::HashMap, fs, path::Path};

use extism::{Function, Plugin};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info, warn};

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

pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub functions: Vec<String>,
    pub plugin_type: PluginType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PluginType {
    Wasm,
    JavaScript,
}

pub struct PluginManager {
    plugins: HashMap<String, Plugin>, // keyed by plugin name (WASM plugins)
    js_plugins: HashMap<String, String>, // keyed by plugin name (JavaScript plugins)
    plugin_info: HashMap<String, PluginInfo>, // metadata for each plugin
    plugin_dir: String, // directory where plugins are stored
}

impl PluginManager {
    /// Scan `$SAGE_PLUGINS` dir and load every `<n>.wasm`+manifest.
    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let mut plugins = HashMap::new();
        let mut plugin_info = HashMap::new();
        let dir_path = dir.as_ref().to_string_lossy().to_string();

        // Create the directory if it doesn't exist
        if !Path::new(&dir_path).exists() {
            fs::create_dir_all(&dir_path)?;
        }

        let mut js_plugins = HashMap::new();

        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();

            // Get the file extension
            let extension = path.extension().and_then(|e| e.to_str());

            // Skip files that are not WASM or JavaScript
            if extension != Some("wasm") && extension != Some("js") {
                continue;
            }

            let _stem = path.file_stem().unwrap().to_string_lossy();
            let manifest_path = path.with_extension("json");

            // Skip if manifest doesn't exist
            if !manifest_path.exists() {
                continue;
            }

            let manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;

            if extension == Some("wasm") {
                // Load WASM plugin
                let wasm = fs::read(&path)?;

                // Try to create the plugin, but don't fail if it fails
                match Plugin::new(wasm, Vec::<Function>::new(), false) {
                    Ok(plugin) => {
                        info!(name = %manifest.name, version = %manifest.version, "loaded WASM plugin");

                        // Store plugin info
                        let info = PluginInfo {
                            name: manifest.name.clone(),
                            version: manifest.version.clone(),
                            functions: manifest.functions.clone(),
                            plugin_type: PluginType::Wasm,
                        };

                        plugins.insert(manifest.name.clone(), plugin);
                        plugin_info.insert(manifest.name, info);
                    },
                    Err(e) => {
                        // Log the error and continue
                        warn!(name = %manifest.name, error = %e, "failed to load WASM plugin");
                    }
                }
            } else if extension == Some("js") {
                // Load JavaScript plugin
                let js_code = fs::read_to_string(&path)?;
                info!(name = %manifest.name, version = %manifest.version, "loaded JavaScript plugin");

                // Store plugin info
                let info = PluginInfo {
                    name: manifest.name.clone(),
                    version: manifest.version.clone(),
                    functions: manifest.functions.clone(),
                    plugin_type: PluginType::JavaScript,
                };

                js_plugins.insert(manifest.name.clone(), js_code);
                plugin_info.insert(manifest.name, info);
            }
        }

        Ok(Self {
            plugins,
            js_plugins,
            plugin_info,
            plugin_dir: dir_path,
        })
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
        // Check if the plugin exists
        let plugin_info = self.plugin_info.get(plugin).ok_or_else(|| PluginError::Manifest("plugin not found"))?;

        match plugin_info.plugin_type {
            PluginType::Wasm => {
                // Get the WASM plugin
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
            },
            PluginType::JavaScript => {
                // Get the JavaScript plugin
                let _js_code = self.js_plugins.get(plugin).ok_or_else(|| PluginError::Manifest("plugin not found"))?;

                // For now, just return a mock response
                // In a real implementation, we would execute the JavaScript code
                let mock_response = format!("JavaScript plugin '{}' called with args: {:?}", plugin, args);
                Ok(mock_response)
            }
        }
    }

    /// List all installed plugins
    pub fn list_plugins(&self) -> Result<Vec<String>> {
        Ok(self.plugin_info.keys().cloned().collect())
    }

    /// Get information about a specific plugin
    pub fn get_plugin_info(&self, name: &str) -> Option<&PluginInfo> {
        self.plugin_info.get(name)
    }

    /// Install a plugin from a WASM file
    pub fn install_plugin(&mut self, path: &str) -> Result<()> {
        let source_path = Path::new(path);
        if !source_path.exists() {
            return Err(PluginError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Plugin file not found: {}", path),
            )));
        }

        // Check if it's a WASM file
        if source_path.extension().and_then(|e| e.to_str()) != Some("wasm") {
            return Err(PluginError::Manifest("Plugin file must have .wasm extension"));
        }

        // Check for manifest file
        let manifest_path = source_path.with_extension("json");
        if !manifest_path.exists() {
            return Err(PluginError::Manifest("Plugin manifest file (.json) not found"));
        }

        // Read and validate manifest
        let manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
        if manifest.name.is_empty() {
            return Err(PluginError::Manifest("Plugin name cannot be empty"));
        }

        // Copy files to plugin directory
        let target_wasm = Path::new(&self.plugin_dir).join(format!("{}.wasm", manifest.name));
        let target_json = Path::new(&self.plugin_dir).join(format!("{}.json", manifest.name));

        fs::copy(source_path, &target_wasm)?;
        fs::copy(manifest_path, &target_json)?;

        // Load the plugin
        let wasm = fs::read(&target_wasm)?;
        let plugin = Plugin::new(wasm, Vec::<Function>::new(), false)?;

        // Store plugin info
        let info = PluginInfo {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            functions: manifest.functions.clone(),
            plugin_type: PluginType::Wasm,
        };

        self.plugins.insert(manifest.name.clone(), plugin);
        self.plugin_info.insert(manifest.name, info);

        Ok(())
    }

    /// Check if a plugin exists
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Get a list of functions supported by a plugin
    pub fn get_plugin_functions(&self, name: &str) -> Option<&Vec<String>> {
        self.plugin_info.get(name).map(|info| &info.functions)
    }
}
