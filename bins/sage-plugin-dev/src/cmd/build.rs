use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use std::process::Command;

#[derive(Args)]
pub struct Build {
    /// Path to the plugin directory
    #[arg(default_value = ".")]
    path: String,

    /// Build in release mode
    #[arg(short, long)]
    release: bool,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        let plugin_dir = Path::new(&self.path);

        // Check if directory exists
        if !plugin_dir.exists() {
            return Err(anyhow!("Directory does not exist: {}", plugin_dir.display()));
        }

        // Check if it's a Rust project
        let cargo_toml = plugin_dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(anyhow!("Not a Rust project: Cargo.toml not found in {}", plugin_dir.display()));
        }

        // Build the plugin
        println!("Building plugin...");

        let profile = if self.release { "release" } else { "debug" };
        let status = Command::new("cargo")
            .current_dir(plugin_dir)
            .args(&["build", "--target", "wasm32-wasip1"])
            .args(if self.release { &["--release"][..] } else { &[][..] })
            .status()?;

        if !status.success() {
            return Err(anyhow!("Failed to build plugin"));
        }

        // Get the plugin name from Cargo.toml
        let cargo_toml_content = std::fs::read_to_string(&cargo_toml)?;
        let plugin_name = extract_package_name(&cargo_toml_content)?;

        // Get the output path
        let output_path = plugin_dir
            .join("target")
            .join("wasm32-wasip1")
            .join(profile)
            .join(format!("{}.wasm", plugin_name.replace('-', "_")));

        println!("Plugin built successfully: {}", output_path.display());
        println!("To install the plugin, run:");
        println!("  sage plugin install {}", output_path.display());

        Ok(())
    }
}

fn extract_package_name(cargo_toml: &str) -> Result<String> {
    for line in cargo_toml.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                return Ok(name.to_string());
            }
        }
    }

    Err(anyhow!("Could not find package name in Cargo.toml"))
}
