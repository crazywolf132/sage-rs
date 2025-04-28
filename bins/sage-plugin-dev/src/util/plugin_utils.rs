use anyhow::{Result, anyhow};
use sage_plugin_api::{PluginManager, PluginInfo, Event};
use std::path::{Path, PathBuf};
use std::fs;

/// Get the default plugin directory, handling platform-specific paths
pub fn get_plugin_dir() -> PathBuf {
    // Get the config directory based on the platform
    let config_dir = if cfg!(target_os = "windows") {
        // On Windows, use %APPDATA%\sage
        match std::env::var("APPDATA") {
            Ok(appdata) => PathBuf::from(appdata).join("sage"),
            Err(_) => {
                // Fallback to user profile directory
                match std::env::var("USERPROFILE") {
                    Ok(profile) => PathBuf::from(profile).join(".config").join("sage"),
                    Err(_) => PathBuf::from("C:\\Users\\Default\\.config\\sage"),
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        // On macOS, use ~/Library/Application Support/sage or ~/.config/sage
        match std::env::var("HOME") {
            Ok(home) => {
                let library_path = PathBuf::from(&home).join("Library/Application Support/sage");
                if library_path.exists() {
                    library_path
                } else {
                    PathBuf::from(&home).join(".config/sage")
                }
            },
            Err(_) => PathBuf::from("~/.config/sage"),
        }
    } else {
        // On Linux and other Unix-like systems, use XDG_CONFIG_HOME or ~/.config
        match std::env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config) => PathBuf::from(xdg_config).join("sage"),
            Err(_) => {
                match std::env::var("HOME") {
                    Ok(home) => PathBuf::from(home).join(".config/sage"),
                    Err(_) => PathBuf::from("~/.config/sage"),
                }
            }
        }
    };

    // Ensure the plugins directory exists
    let plugins_dir = config_dir.join("plugins");
    if !plugins_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&plugins_dir) {
            eprintln!("Warning: Failed to create plugins directory: {}", e);
        }
    }

    plugins_dir
}

/// Load a plugin from a specific path
pub fn load_plugin(path: &Path) -> Result<(PluginManager, String)> {
    // Check if the file exists
    if !path.exists() {
        return Err(anyhow!("Plugin file not found: {}", path.display()));
    }

    // Get the file extension
    let extension = path.extension().and_then(|e| e.to_str());

    // Check if it's a WASM or JS file
    if extension != Some("wasm") && extension != Some("js") {
        return Err(anyhow!("Plugin file must have .wasm or .js extension"));
    }

    // Check for manifest file
    let manifest_path = path.with_extension("json");
    if !manifest_path.exists() {
        return Err(anyhow!("Plugin manifest file (.json) not found"));
    }

    // Read the manifest to get the plugin name
    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content)?;
    let plugin_name = manifest["name"].as_str()
        .ok_or_else(|| anyhow!("Plugin manifest missing 'name' field"))?
        .to_string();

    // Create a temporary directory to load the plugin
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Copy the plugin and manifest to the temporary directory
    let temp_plugin_path = temp_path.join(format!("{}.{}", plugin_name, extension.unwrap()));
    let temp_manifest_path = temp_path.join(format!("{}.json", plugin_name));

    fs::copy(path, &temp_plugin_path)?;
    fs::copy(manifest_path, &temp_manifest_path)?;

    // Load the plugin
    let plugin_manager = PluginManager::load_dir(temp_path)?;

    Ok((plugin_manager, plugin_name))
}

/// Test a plugin with a pre-push event
pub fn test_pre_push(plugin_manager: &mut PluginManager, plugin_name: &str, branch: &str, debug: bool) -> Result<String> {
    // Check if the plugin has the pre_push function
    let functions = plugin_manager.get_plugin_functions(plugin_name)
        .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

    if !functions.contains(&"pre_push".to_string()) {
        return Err(anyhow!("Plugin does not support pre_push function"));
    }

    // Create the event
    let event = Event::PrePush { branch: branch.to_string() };
    let _input = serde_json::to_vec(&event)?; // Unused but kept for documentation

    if debug {
        println!("DEBUG: Sending pre-push event to plugin");
        println!("DEBUG: Branch: {}", branch);
        println!("DEBUG: Event JSON: {}", serde_json::to_string_pretty(&event)?);
    }

    // Call the plugin
    let start_time = std::time::Instant::now();
    let result = match plugin_manager.cli(plugin_name, &[branch.to_string()]) {
        Ok(output) => {
            if debug {
                println!("DEBUG: Plugin execution successful");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Raw output: {}", output);

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("DEBUG: Parsed output: {}", serde_json::to_string_pretty(&json)?);
                }
            }
            format!("Success: {}", output)
        },
        Err(e) => {
            if debug {
                println!("DEBUG: Plugin execution failed");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Error: {}", e);
            }
            format!("Error: {}", e)
        },
    };

    Ok(result)
}

/// Test a plugin with a post-commit event
pub fn test_post_commit(plugin_manager: &mut PluginManager, plugin_name: &str, commit_id: &str, debug: bool) -> Result<String> {
    // Check if the plugin has the post_commit function
    let functions = plugin_manager.get_plugin_functions(plugin_name)
        .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

    if !functions.contains(&"post_commit".to_string()) {
        return Err(anyhow!("Plugin does not support post_commit function"));
    }

    // Create the event
    let event = Event::PostCommit { oid: commit_id.to_string() };
    let _input = serde_json::to_vec(&event)?; // Unused but kept for documentation

    if debug {
        println!("DEBUG: Sending post-commit event to plugin");
        println!("DEBUG: Commit ID: {}", commit_id);
        println!("DEBUG: Event JSON: {}", serde_json::to_string_pretty(&event)?);
    }

    // Call the plugin
    let start_time = std::time::Instant::now();
    let result = match plugin_manager.cli(plugin_name, &[commit_id.to_string()]) {
        Ok(output) => {
            if debug {
                println!("DEBUG: Plugin execution successful");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Raw output: {}", output);

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("DEBUG: Parsed output: {}", serde_json::to_string_pretty(&json)?);
                }
            }
            format!("Success: {}", output)
        },
        Err(e) => {
            if debug {
                println!("DEBUG: Plugin execution failed");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Error: {}", e);
            }
            format!("Error: {}", e)
        },
    };

    Ok(result)
}

/// Test a plugin with CLI arguments
pub fn test_run(plugin_manager: &mut PluginManager, plugin_name: &str, args: &[String], debug: bool) -> Result<String> {
    // Check if the plugin has the run function
    let functions = plugin_manager.get_plugin_functions(plugin_name)
        .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

    if !functions.contains(&"run".to_string()) {
        return Err(anyhow!("Plugin does not support run function"));
    }

    if debug {
        println!("DEBUG: Executing plugin CLI command");
        println!("DEBUG: Plugin: {}", plugin_name);
        println!("DEBUG: Arguments: {:?}", args);
    }

    // Call the plugin
    let start_time = std::time::Instant::now();
    let result = match plugin_manager.cli(plugin_name, args) {
        Ok(output) => {
            if debug {
                println!("DEBUG: Plugin execution successful");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Raw output: {}", output);

                // Try to parse the output as JSON for better display
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("DEBUG: Parsed output: {}", serde_json::to_string_pretty(&json)?);
                }
            }
            format!("Success: {}", output)
        },
        Err(e) => {
            if debug {
                println!("DEBUG: Plugin execution failed");
                println!("DEBUG: Execution time: {:?}", start_time.elapsed());
                println!("DEBUG: Error: {}", e);
            }
            format!("Error: {}", e)
        },
    };

    Ok(result)
}

/// Validate a plugin's structure and manifest
pub fn validate_plugin(path: &Path) -> Result<()> {
    // Check if the file exists
    if !path.exists() {
        return Err(anyhow!("Plugin file not found: {}", path.display()));
    }

    // Get the file extension
    let extension = path.extension().and_then(|e| e.to_str());

    // Check if it's a WASM or JS file
    if extension != Some("wasm") && extension != Some("js") {
        return Err(anyhow!("Plugin file must have .wasm or .js extension"));
    }

    // Check for manifest file
    let manifest_path = path.with_extension("json");
    if !manifest_path.exists() {
        return Err(anyhow!("Plugin manifest file (.json) not found"));
    }

    // Read and validate manifest
    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content)?;

    // Check required fields
    if !manifest.get("name").is_some_and(|v| v.is_string()) {
        return Err(anyhow!("Plugin manifest missing 'name' field"));
    }

    if !manifest.get("version").is_some_and(|v| v.is_string()) {
        return Err(anyhow!("Plugin manifest missing 'version' field"));
    }

    if !manifest.get("functions").is_some_and(|v| v.is_array()) {
        return Err(anyhow!("Plugin manifest missing 'functions' array"));
    }

    // Validate functions
    let functions = manifest["functions"].as_array().unwrap();
    if functions.is_empty() {
        return Err(anyhow!("Plugin manifest has empty 'functions' array"));
    }

    for func in functions {
        if !func.is_string() {
            return Err(anyhow!("Plugin manifest 'functions' array contains non-string value"));
        }

        let func_name = func.as_str().unwrap();
        if !["pre_push", "post_commit", "run"].contains(&func_name) {
            return Err(anyhow!("Plugin manifest contains unknown function: {}", func_name));
        }
    }

    Ok(())
}

/// Get detailed information about a plugin
pub fn get_plugin_info<'a>(plugin_manager: &'a PluginManager, plugin_name: &str) -> Result<&'a PluginInfo> {
    let info = plugin_manager.get_plugin_info(plugin_name)
        .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

    Ok(info)
}

/// Clean up unnecessary files from the plugins directory
pub fn clean_plugins_directory() -> Result<Vec<String>> {
    let plugins_dir = get_plugin_dir();
    let mut removed_files = Vec::new();

    // Check if the directory exists
    if !plugins_dir.exists() {
        return Ok(removed_files);
    }

    // Track valid plugin files
    let mut valid_plugins = std::collections::HashSet::new();
    let mut valid_manifests = std::collections::HashSet::new();

    // First pass: identify valid plugins and their manifests
    for entry_result in fs::read_dir(&plugins_dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|e| e.to_str());

        // Check for WASM or JS files
        if extension == Some("wasm") || extension == Some("js") {
            // Check if there's a corresponding manifest
            let manifest_path = path.with_extension("json");
            if manifest_path.exists() {
                // This is a valid plugin with a manifest
                valid_plugins.insert(path.clone());
                valid_manifests.insert(manifest_path);
            }
        }
    }

    // Second pass: remove files that aren't valid plugins or manifests
    for entry_result in fs::read_dir(&plugins_dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Skip if it's a valid plugin or manifest
        if valid_plugins.contains(&path) || valid_manifests.contains(&path) {
            continue;
        }

        // Remove the file
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("Warning: Failed to remove file {}: {}", path.display(), e);
        } else {
            removed_files.push(path.file_name().unwrap().to_string_lossy().to_string());
        }
    }

    Ok(removed_files)
}
