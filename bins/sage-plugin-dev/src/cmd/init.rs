use anyhow::{Result, anyhow};
use clap::Args;
use std::fs;
use std::path::Path;
use crate::templates::{rust, javascript};

#[derive(Args)]
pub struct Init {
    /// Name of the plugin
    pub name: String,

    /// Type of plugin to create (rust or js)
    #[arg(short, long, default_value = "rust")]
    pub plugin_type: String,

    /// Directory to create the plugin in (defaults to current directory)
    #[arg(short, long)]
    pub dir: Option<String>,
}

impl Init {
    pub fn run(&self) -> Result<()> {
        let plugin_dir = match &self.dir {
            Some(dir) => Path::new(dir).join(&self.name),
            None => Path::new(&self.name).to_path_buf(),
        };

        // Check if directory already exists
        if plugin_dir.exists() {
            return Err(anyhow!("Directory already exists: {}", plugin_dir.display()));
        }

        // Create the directory
        fs::create_dir_all(&plugin_dir)?;

        match self.plugin_type.as_str() {
            "rust" => self.create_rust_plugin(&plugin_dir)?,
            "js" => self.create_js_plugin(&plugin_dir)?,
            _ => return Err(anyhow!("Unsupported plugin type: {}", self.plugin_type)),
        }

        println!("✅ Plugin project created at: {}", plugin_dir.display());
        println!("\nNext steps:");

        if self.plugin_type == "rust" {
            println!("  1. cd {}", plugin_dir.display());
            println!("  2. Edit src/lib.rs to implement your plugin logic");
            println!("  3. Build with: cargo build --target wasm32-wasip1 --release");
            println!("  4. Test with: sage-plugin-dev run target/wasm32-wasip1/release/{}.wasm", self.name.replace('-', "_"));
            println!("  5. Install with: sage plugin install target/wasm32-wasip1/release/{}.wasm", self.name.replace('-', "_"));
        } else {
            println!("  1. cd {}", plugin_dir.display());
            println!("  2. Edit index.js to implement your plugin logic");
            println!("  3. Test with: sage-plugin-dev run index.js");
            println!("  4. Install with: sage plugin install index.js");
        }

        println!("\nSee README.md for more detailed instructions.");

        Ok(())
    }

    fn create_rust_plugin(&self, plugin_dir: &Path) -> Result<()> {
        // Create src directory
        let src_dir = plugin_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        // Create Cargo.toml
        fs::write(
            plugin_dir.join("Cargo.toml"),
            rust::cargo_toml_template(&self.name),
        )?;

        // Create src/lib.rs
        fs::write(
            src_dir.join("lib.rs"),
            rust::lib_rs_template(&self.name),
        )?;

        // Create manifest.json
        fs::write(
            plugin_dir.join(format!("{}.json", self.name)),
            rust::manifest_json_template(&self.name),
        )?;

        // Create README.md
        fs::write(
            plugin_dir.join("README.md"),
            rust::readme_md_template(&self.name),
        )?;

        // Create .gitignore
        fs::write(
            plugin_dir.join(".gitignore"),
            rust::gitignore_template(),
        )?;

        Ok(())
    }

    fn create_js_plugin(&self, plugin_dir: &Path) -> Result<()> {
        // Create index.js
        fs::write(
            plugin_dir.join("index.js"),
            javascript::index_js_template(&self.name),
        )?;

        // Create manifest.json
        fs::write(
            plugin_dir.join(format!("{}.json", self.name)),
            javascript::manifest_json_template(&self.name),
        )?;

        // Create README.md
        fs::write(
            plugin_dir.join("README.md"),
            javascript::readme_md_template(&self.name),
        )?;

        // Create .gitignore
        fs::write(
            plugin_dir.join(".gitignore"),
            javascript::gitignore_template(),
        )?;

        Ok(())
    }
}
