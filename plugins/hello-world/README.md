# Hello World Plugin

A simple example plugin for Sage that demonstrates the basic functionality of the plugin API.

## Features

This plugin demonstrates:

1. Handling pre-push hooks
2. Handling post-commit hooks
3. Implementing a CLI command

## Building

To build this plugin:

```bash
cd plugins/hello-world
rustup target add wasm32-wasip1  # Only needed once
cargo build --target wasm32-wasip1 --release
```

This will create a WASM file at `target/wasm32-wasip1/release/hello_world_plugin.wasm`.

## Installing

To install the plugin:

1. Copy the WASM file and JSON manifest to your Sage plugins directory:

```bash
mkdir -p ~/.config/sage/plugins
cp target/wasm32-wasip1/release/hello_world_plugin.wasm ~/.config/sage/plugins/hello-world.wasm
cp hello-world.json ~/.config/sage/plugins/hello-world.json
```

2. Or use the Sage plugin install command:

```bash
sage plugin install target/wasm32-wasip1/release/hello_world_plugin.wasm
```

## Usage

### CLI Command

Run the plugin as a CLI command:

```bash
sage plugin hello-world
```

With arguments:

```bash
sage plugin hello-world YourName
```

### Git Hooks

The plugin will automatically:

1. Block pushes directly to the main branch
2. Log information about commits when they are created

## Code Structure

- `src/lib.rs`: The plugin implementation
- `hello-world.json`: The plugin manifest
- `Cargo.toml`: The build configuration
