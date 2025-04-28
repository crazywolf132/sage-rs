# Sage Plugins

This directory contains example plugins for Sage.

## Available Plugins

### Rust Plugins
- [hello-world](./hello-world): A simple plugin demonstrating basic functionality
- [commit-linter](./commit-linter): A plugin that validates commit messages according to Conventional Commits

### JavaScript Plugins
- [branch-validator](./branch-validator): A plugin that validates branch names according to common naming conventions
- [git-stats](./git-stats): A plugin that provides statistics about your git repository

## Creating Your Own Plugin

See the [Plugin API documentation](../crates/plugin-api/README.md) for details on how to create your own plugins.

## Building and Installing Plugins

### Building All Plugins

You can build all plugins (Rust and TypeScript) using the provided build script:

```bash
cd plugins
./build-all.sh
```

This will build all plugins and install them to `~/.config/sage/plugins/`.

### Installing JavaScript Plugins

If you only want to install the JavaScript plugins without building TypeScript plugins, you can use:

```bash
cd plugins
./install-js-plugins.sh
```

This will copy the JavaScript plugins to `~/.config/sage/plugins/`.

### Building Individual Plugins

#### Rust Plugins

Each Rust plugin directory contains a Cargo.toml file and can be built with:

```bash
cd plugins/<plugin-name>
rustup target add wasm32-wasip1  # Only needed once
cargo build --target wasm32-wasip1 --release
```

The built plugin will be at `target/wasm32-wasip1/release/<plugin_name>_plugin.wasm`.

#### JavaScript Plugins

JavaScript plugins don't need to be built. You can simply copy the JavaScript file to the plugins directory:

```bash
cp plugins/<plugin-name>/index.js ~/.config/sage/plugins/<plugin-name>.js
cp plugins/<plugin-name>/<plugin-name>.json ~/.config/sage/plugins/
```

## Installing Plugins

Plugins can be installed with:

```bash
sage plugin install <path-to-wasm-file>
```

Or manually by copying the `.wasm` and `.json` files to `~/.config/sage/plugins/`.

## Plugin Structure

Each plugin consists of:

1. A WebAssembly (`.wasm`) file containing the compiled code, or a JavaScript (`.js`) file
2. A JSON manifest (`.json`) file with metadata

## Plugin Manifest

The manifest file should contain:

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
