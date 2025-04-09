#!/bin/bash
# Script to update version in Cargo.toml without causing merge conflicts

set -e

# Check if a version is provided
if [ $# -ne 1 ]; then
  echo "Usage: $0 <new_version>"
  echo "Example: $0 0.2.11"
  exit 1
fi

NEW_VERSION=$1

# Validate version format
if [[ ! "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo "Error: Version format should be X.Y.Z or X.Y.Z-suffix"
  exit 1
fi

# Update Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
cargo update -p sage

echo "Version updated to $NEW_VERSION"
echo "Remember to commit these changes separately from feature work to minimize conflicts"
