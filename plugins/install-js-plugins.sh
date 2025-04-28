#!/bin/bash
# Script to install JavaScript plugins

set -e

# Create plugins directory if it doesn't exist
PLUGIN_DIR="$HOME/.config/sage/plugins"
mkdir -p "$PLUGIN_DIR"

# Copy branch-validator plugin
echo "Installing branch-validator plugin..."
cp branch-validator/index.js "$PLUGIN_DIR/branch-validator.js"
cp branch-validator/branch-validator.json "$PLUGIN_DIR/"
echo "branch-validator plugin installed successfully!"

# Copy git-stats plugin
echo "Installing git-stats plugin..."
cp git-stats/index.js "$PLUGIN_DIR/git-stats.js"
cp git-stats/git-stats.json "$PLUGIN_DIR/"
echo "git-stats plugin installed successfully!"

echo "All JavaScript plugins installed successfully!"
