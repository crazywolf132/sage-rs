use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use notify::{Watcher, RecursiveMode, Event};

#[derive(Args)]
pub struct Watch {
    /// Path to the plugin directory
    #[arg(default_value = ".")]
    path: String,

    /// Build in release mode
    #[arg(short, long)]
    release: bool,

    /// Automatically test the plugin after building
    #[arg(short, long)]
    test: bool,
}

impl Watch {
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

        // Get the plugin name from Cargo.toml
        let cargo_toml_content = std::fs::read_to_string(&cargo_toml)?;
        let plugin_name = extract_package_name(&cargo_toml_content)?;

        println!("Watching for changes in {}...", plugin_dir.display());
        println!("Press Ctrl+C to stop");

        // Set up file watcher
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or(());
            }
        })?;

        // Watch the src directory
        let src_dir = plugin_dir.join("src");
        watcher.watch(&src_dir, RecursiveMode::Recursive)?;

        // Initial build
        self.build_plugin(plugin_dir, &plugin_name)?;

        // Keep track of the last build time to debounce events
        let mut last_build = Instant::now();

        // Watch for changes
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    // Only rebuild if it's been at least 1 second since the last build
                    // This helps debounce multiple file change events
                    if event.kind.is_modify() && last_build.elapsed() > Duration::from_secs(1) {
                        println!("\nChange detected, rebuilding...");
                        if let Err(e) = self.build_plugin(plugin_dir, &plugin_name) {
                            println!("Build failed: {}", e);
                        }
                        last_build = Instant::now();
                    }
                },
                Err(_) => {
                    // Timeout, continue
                }
            }
        }
    }

    fn build_plugin(&self, plugin_dir: &Path, plugin_name: &str) -> Result<()> {
        let profile = if self.release { "release" } else { "debug" };
        let status = Command::new("cargo")
            .current_dir(plugin_dir)
            .args(&["build", "--target", "wasm32-wasip1"])
            .args(if self.release { &["--release"][..] } else { &[][..] })
            .status()?;

        if !status.success() {
            return Err(anyhow!("Failed to build plugin"));
        }

        // Get the output path
        let output_path = plugin_dir
            .join("target")
            .join("wasm32-wasip1")
            .join(profile)
            .join(format!("{}.wasm", plugin_name.replace('-', "_")));

        println!("Build successful: {}", output_path.display());

        // Test the plugin if requested
        if self.test {
            println!("Testing plugin...");

            // Test the run function
            let status = Command::new("sage-plugin-dev")
                .args(&["test", "run", &output_path.to_string_lossy()])
                .status()?;

            if !status.success() {
                println!("Test failed");
            }
        }

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
