# Sage Plugin API

This crate provides a plugin system for Sage using WebAssembly (WASM) via Extism.

## Overview

The plugin system allows extending Sage with custom functionality through WASM plugins.
Plugins can:

- Hook into git lifecycle events (pre-push, post-commit)
- Add custom CLI commands
- Integrate with any part of the application

## Plugin Structure

Each plugin consists of two files:
- A `.wasm` file containing the compiled WebAssembly code
- A `.json` manifest file with metadata and configuration

## Creating Plugins

Plugins can be created in any language that compiles to WebAssembly, including:
- Rust (with `wasm-bindgen` or `wasm-pack`)
- AssemblyScript
- C/C++ (with Emscripten)
- Go (with TinyGo)

### Creating a Plugin with Rust

1. Create a new library crate:

```bash
cargo new --lib my-plugin
```

2. Configure it as a WASM library in `Cargo.toml`:

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
extism-pdk = "1.0.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

3. Implement your plugin in `src/lib.rs`:

```rust
use extism_pdk::*;
use serde::{Deserialize, Serialize};

// Define the event structure that matches the host's Event enum
#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum Event {
    PrePush { branch: String },
    PostCommit { oid: String },
}

// Define the reply structure that matches the host's Reply enum
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Reply {
    Ok { message: String },
    Error { message: String },
}

// Define the CLI args structure
#[derive(Deserialize)]
struct CliArgs {
    args: Vec<String>,
}

// Pre-push hook
#[plugin_fn]
pub fn pre_push(input: String) -> FnResult<String> {
    let event: Event = serde_json::from_str(&input)?;
    
    match event {
        Event::PrePush { branch } => {
            // Your validation logic here
            let reply = Reply::Ok { 
                message: format!("Pre-push check passed for branch: {}", branch) 
            };
            Ok(serde_json::to_string(&reply)?)
        },
        _ => {
            let reply = Reply::Error { 
                message: "Unexpected event type".into() 
            };
            Ok(serde_json::to_string(&reply)?)
        }
    }
}

// CLI command
#[plugin_fn]
pub fn run(input: String) -> FnResult<String> {
    let cli_args: CliArgs = serde_json::from_str(&input)?;
    
    let message = format!("Hello from my plugin! Args: {:?}", cli_args.args);
    
    let reply = Reply::Ok { message };
    Ok(serde_json::to_string(&reply)?)
}
```

4. Create a manifest file `my-plugin.json`:

```json
{
  "name": "my-plugin",
  "version": "0.1.0",
  "functions": ["pre_push", "run"]
}
```

5. Build your plugin:

```bash
cargo build --target wasm32-wasi --release
```

6. Install your plugin:

```bash
sage plugin install target/wasm32-wasi/release/my_plugin.wasm
```

## Plugin Manifest

The manifest file (`.json`) should contain:

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "functions": ["pre_push", "post_commit", "run"]
}
```

## Plugin Functions

Plugins can export the following functions:

- `pre_push`: Called before pushing to a remote
- `post_commit`: Called after a commit is created
- `run`: Called when the plugin is executed as a CLI command

## Using Plugins

Once installed, plugins can be used in several ways:

1. Git hooks are automatically triggered during git operations
2. CLI commands can be run with `sage plugin <name> [args...]`
3. List installed plugins with `sage plugin list`

## Plugin API Reference

### Event Types

Events sent to plugins:

```rust
enum Event {
    PrePush { branch: String },
    PostCommit { oid: String },
}
```

### Reply Types

Replies from plugins:

```rust
enum Reply {
    Ok { message: String },
    Error { message: String },
}
```

## Example Plugins

Check out the `plugins/` directory for example plugins:

- `hello-world`: A simple plugin demonstrating basic functionality
