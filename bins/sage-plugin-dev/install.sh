#!/bin/bash

set -e

echo "Installing sage-plugin-dev..."

# Build and install the binary
cargo install --path .

# Documentation
echo "Documentation available in README.md"

echo "Installation complete!"
echo "Run 'sage-plugin-dev --help' to get started."
