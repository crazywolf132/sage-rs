#!/bin/bash
# Build script for TypeScript plugins

set -e

# Function to build a TypeScript plugin
build_ts_plugin() {
  local plugin_dir=$1
  echo "Building TypeScript plugin: $plugin_dir"

  cd "$plugin_dir"

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
echo "Building TypeScript plugins..."

# Create plugins directory if it doesn't exist
mkdir -p ~/.config/sage/plugins

# Build TypeScript plugins
build_ts_plugin "branch-validator"
build_ts_plugin "git-stats"

echo "TypeScript plugins built and installed successfully!"
