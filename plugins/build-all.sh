#!/bin/bash
# Build script for all Sage plugins

set -e

# Function to build a Rust plugin
build_rust_plugin() {
  local plugin_dir=$1
  echo "Building Rust plugin: $plugin_dir"

  cd "$plugin_dir"

  # Check if Rust toolchain for wasm32-wasip1 is installed
  if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
    echo "Installing wasm32-wasip1 target..."
    rustup target add wasm32-wasip1
  fi

  # Build the plugin
  cargo build --target wasm32-wasip1 --release

  # Get the actual file name (convert hyphens to underscores)
  local wasm_file=$(echo "${plugin_dir}_plugin.wasm" | tr '-' '_')

  # Copy the built plugin to the plugin directory
  PLUGIN_DIR="$HOME/.config/sage/plugins"
  mkdir -p "$PLUGIN_DIR"
  cp "target/wasm32-wasip1/release/$wasm_file" "$PLUGIN_DIR/${plugin_dir}.wasm"
  cp "${plugin_dir}.json" "$PLUGIN_DIR/"

  echo "Plugin $plugin_dir built and installed successfully!"
  cd ..
}

# Function to build a TypeScript plugin
build_ts_plugin() {
  local plugin_dir=$1
  echo "Building TypeScript plugin: $plugin_dir"

  cd "$plugin_dir"

  # Check if extism-js is installed
  if ! command -v extism-js &> /dev/null; then
    echo "Installing extism-js..."
    pnpm install -g @extism/js
  fi

  # Install dependencies and build
  pnpm install
  pnpm run build

  # Copy the built plugin to the plugin directory
  PLUGIN_DIR="$HOME/.config/sage/plugins"
  mkdir -p "$PLUGIN_DIR"
  cp "${plugin_dir}.wasm" "$PLUGIN_DIR/"
  cp "${plugin_dir}.json" "$PLUGIN_DIR/"

  echo "Plugin $plugin_dir built and installed successfully!"
  cd ..
}

# Main script
echo "Building all Sage plugins..."

# Create plugins directory if it doesn't exist
mkdir -p ~/.config/sage/plugins

# Build Rust plugins
build_rust_plugin "hello-world"
build_rust_plugin "commit-linter"

# Build TypeScript plugins
build_ts_plugin "branch-validator"
build_ts_plugin "git-stats"

echo "All plugins built and installed successfully!"
