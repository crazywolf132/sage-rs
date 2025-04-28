/// Template for Cargo.toml
pub fn cargo_toml_template(name: &str) -> String {
    format!(
        r#"[package]
name = "{0}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
extism-pdk = "1.0.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#,
        name
    )
}

/// Template for lib.rs
pub fn lib_rs_template(name: &str) -> String {
    format!(
        r#"use extism_pdk::*;
use serde::{{Deserialize, Serialize}};

// Define the event structure that matches the host's Event enum
#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum Event {{
    PrePush {{ branch: String }},
    PostCommit {{ oid: String }},
}}

// Define the reply structure that matches the host's Reply enum
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Reply {{
    Ok {{ message: String }},
    Error {{ message: String }},
}}

// Define the CLI args structure
#[derive(Deserialize)]
struct CliArgs {{
    args: Vec<String>
}}

// Pre-push hook
#[plugin_fn]
pub fn pre_push(input: String) -> FnResult<String> {{
    let event: Event = serde_json::from_str(&input)?;

    match event {{
        Event::PrePush {{ branch }} => {{
            // Your validation logic here
            let reply = Reply::Ok {{
                message: format!("Pre-push check passed for branch: {{}}", branch)
            }};
            Ok(serde_json::to_string(&reply)?)
        }},
        _ => {{
            let reply = Reply::Error {{
                message: "Unexpected event type".into()
            }};
            Ok(serde_json::to_string(&reply)?)
        }}
    }}
}}

// Post-commit hook
#[plugin_fn]
pub fn post_commit(input: String) -> FnResult<String> {{
    let event: Event = serde_json::from_str(&input)?;

    match event {{
        Event::PostCommit {{ oid }} => {{
            // Your post-commit logic here
            let reply = Reply::Ok {{
                message: format!("Post-commit check passed for commit: {{}}", oid)
            }};
            Ok(serde_json::to_string(&reply)?)
        }},
        _ => {{
            let reply = Reply::Error {{
                message: "Unexpected event type".into()
            }};
            Ok(serde_json::to_string(&reply)?)
        }}
    }}
}}

// CLI command
#[plugin_fn]
pub fn run(input: String) -> FnResult<String> {{
    let cli_args: CliArgs = serde_json::from_str(&input)?;

    let message = if cli_args.args.is_empty() {{
        format!("Hello from {{}}! This is a Sage plugin.\n\nUsage: sage plugin {{}} [args...]\n", "{0}", "{0}")
    }} else {{
        format!("Hello from {{}}! Args: {{:?}}", "{0}", cli_args.args)
    }};

    let reply = Reply::Ok {{ message }};
    Ok(serde_json::to_string(&reply)?)
}}
"#,
        name
    )
}

/// Template for manifest.json
pub fn manifest_json_template(name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "functions": ["pre_push", "post_commit", "run"]
}}"#,
        name
    )
}

/// Template for README.md
pub fn readme_md_template(name: &str) -> String {
    format!(
        r#"# {} Plugin

A plugin for Sage.

## Features

This plugin demonstrates:

1. Handling pre-push hooks
2. Handling post-commit hooks
3. Implementing a CLI command

## Building

To build this plugin:

```bash
cd {}
rustup target add wasm32-wasip1  # Only needed once
cargo build --target wasm32-wasip1 --release
```

This will create a WASM file at `target/wasm32-wasip1/release/{}.wasm`.

## Installing

To install the plugin:

```bash
mkdir -p ~/.config/sage/plugins
cp target/wasm32-wasip1/release/{}.wasm ~/.config/sage/plugins/{}.wasm
cp {}.json ~/.config/sage/plugins/{}.json
```

Or use the Sage plugin install command:

```bash
sage plugin install target/wasm32-wasip1/release/{}.wasm
```

## Usage

### CLI Command

Run the plugin as a CLI command:

```bash
sage plugin {}
```

With arguments:

```bash
sage plugin {} arg1 arg2
```

### Git Hooks

The plugin will automatically:

1. Run pre-push checks when pushing to a remote
2. Run post-commit checks when creating a commit
"#,
        name, name, name, name, name, name, name, name, name, name
    )
}

/// Template for .gitignore
pub fn gitignore_template() -> String {
    r#"/target
**/*.rs.bk
Cargo.lock
"#.to_string()
}
