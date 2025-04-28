# Sage Plugin Developer Tools

A powerful CLI tool for testing, debugging, and benchmarking Sage plugins.

<div align="center">
  <img src="https://via.placeholder.com/800x200?text=Sage+Plugin+Developer+Tools" alt="Sage Plugin Developer Tools" width="800"/>
</div>

## Overview

The `sage-plugin-dev` tool is designed to help plugin developers create, test, and debug their Sage plugins with ease. It provides a comprehensive set of features that simulate the Sage environment, allowing you to:

- Test how your plugins respond to Git events (pre-push, post-commit)
- Debug plugin execution with detailed tracing and timing information
- Benchmark plugin performance under various conditions
- Validate plugin structure and functionality
- Test with real Git repository data or mock scenarios

Whether you're developing a simple plugin or a complex one with multiple functions, this tool provides everything you need to ensure your plugin works correctly and efficiently.

## Features

| Feature | Description |
|---------|-------------|
| 🧪 **Testing** | Simulate Sage events (pre-push, post-commit) with customizable inputs |
| 🔍 **Debugging** | View detailed execution traces, inputs, and outputs |
| ⚡ **Benchmarking** | Measure performance with statistical analysis |
| ✅ **Validation** | Ensure plugins meet Sage's requirements and structure |
| 🔄 **Real Git Integration** | Test with actual data from your Git repository |
| 🧩 **Mock Scenarios** | Test edge cases with predefined data patterns |
| 📊 **Detailed Reports** | Get comprehensive information about your plugins |
| 🛠️ **Templates** | Quickly create new plugin projects |

## Installation

### Prerequisites

- Rust and Cargo installed
- WASM target installed: `rustup target add wasm32-wasip1`
- Sage CLI installed

### Install from Source

```bash
# From the sage repository root
cargo install --path bins/sage-plugin-dev
```

### Verify Installation

```bash
sage-plugin-dev --version
```

### Quick Reference

The command reference section below provides details on all available commands and options.

## Quick Start Guide

This quick start guide will walk you through the process of testing an existing plugin:

### 1. Basic Plugin Testing

```bash
# Test a plugin's CLI command
sage-plugin-dev run path/to/plugin.wasm

# Test with arguments
sage-plugin-dev run path/to/plugin.wasm arg1 arg2 --verbose
```

### 2. Testing Git Hooks

```bash
# Test pre-push hook
sage-plugin-dev pre-push path/to/plugin.wasm

# Test post-commit hook
sage-plugin-dev post-commit path/to/plugin.wasm
```

### 3. Advanced Testing

```bash
# Test with real git data
sage-plugin-dev git-hook path/to/plugin.wasm --hook-type pre-push --real-data

# Trace plugin execution
sage-plugin-dev trace path/to/plugin.wasm --function pre-push

# Benchmark plugin performance
sage-plugin-dev benchmark path/to/plugin.wasm --function run --iterations 100
```

### 4. Validation and Information

```bash
# Validate plugin structure and functionality
sage-plugin-dev validate path/to/plugin.wasm

# Get detailed plugin information
sage-plugin-dev info path/to/plugin.wasm --verbose
```

## Command Reference

This section provides detailed information about each command and its options.

### Basic Commands

<details>
<summary><b>🧪 pre-push</b> - Test a plugin's pre-push hook</summary>

Tests how a plugin responds to a pre-push Git event.

```bash
sage-plugin-dev pre-push [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--branch <BRANCH>`: Branch name to use in the test [default: main]
- `--verbose`: Show detailed output from the plugin
- `--debug`: Show debug information including timing and raw data

**Examples:**
```bash
# Test with default branch (main)
sage-plugin-dev pre-push path/to/plugin.wasm

# Test with a specific branch
sage-plugin-dev pre-push path/to/plugin.wasm --branch feature/my-branch

# Show detailed output
sage-plugin-dev pre-push path/to/plugin.wasm --verbose

# Show debug information
sage-plugin-dev pre-push path/to/plugin.wasm --debug
```
</details>

<details>
<summary><b>🧪 post-commit</b> - Test a plugin's post-commit hook</summary>

Tests how a plugin responds to a post-commit Git event.

```bash
sage-plugin-dev post-commit [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--commit-id <COMMIT_ID>`: Commit ID to use in the test [default: abcdef1234567890]
- `--verbose`: Show detailed output from the plugin
- `--debug`: Show debug information including timing and raw data

**Examples:**
```bash
# Test with default commit ID
sage-plugin-dev post-commit path/to/plugin.wasm

# Test with a specific commit ID
sage-plugin-dev post-commit path/to/plugin.wasm --commit-id abc123

# Show detailed output
sage-plugin-dev post-commit path/to/plugin.wasm --verbose

# Show debug information
sage-plugin-dev post-commit path/to/plugin.wasm --debug
```
</details>

<details>
<summary><b>🧪 run</b> - Test a plugin's CLI command</summary>

Tests how a plugin responds to CLI commands.

```bash
sage-plugin-dev run [OPTIONS] <PATH> [ARGS]...
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)
- `[ARGS]...`: Arguments to pass to the plugin

**Options:**
- `--verbose`: Show detailed output from the plugin
- `--debug`: Show debug information including timing and raw data

**Examples:**
```bash
# Run with no arguments
sage-plugin-dev run path/to/plugin.wasm

# Run with arguments
sage-plugin-dev run path/to/plugin.wasm arg1 arg2

# Show detailed output
sage-plugin-dev run path/to/plugin.wasm --verbose

# Show debug information
sage-plugin-dev run path/to/plugin.wasm arg1 --debug
```
</details>

<details>
<summary><b>📊 info</b> - Get plugin information</summary>

Displays detailed information about a plugin.

```bash
sage-plugin-dev info [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--verbose`: Show detailed information including manifest content

**Examples:**
```bash
# Basic information
sage-plugin-dev info path/to/plugin.wasm

# Detailed information including manifest
sage-plugin-dev info path/to/plugin.wasm --verbose
```
</details>

<details>
<summary><b>✅ validate</b> - Validate a plugin</summary>

Validates a plugin's structure and functionality.

```bash
sage-plugin-dev validate [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--verbose`: Show detailed validation information

**Examples:**
```bash
# Validate plugin structure and functionality
sage-plugin-dev validate path/to/plugin.wasm

# Show detailed validation information
sage-plugin-dev validate path/to/plugin.wasm --verbose
```
</details>

### Advanced Commands

<details>
<summary><b>🔄 git-hook</b> - Test with real Git data</summary>

Simulates a Git hook with real repository data.

```bash
sage-plugin-dev git-hook [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--hook-type <HOOK_TYPE>`: Type of git hook to simulate (pre-push or post-commit) [default: post-commit]
- `--verbose`: Show detailed output from the plugin
- `--debug`: Show debug information
- `--real-data`: Use the current git repository for real data

**Examples:**
```bash
# Test pre-push with data from your current git repository
sage-plugin-dev git-hook path/to/plugin.wasm --hook-type pre-push --real-data

# Test post-commit with data from your current git repository
sage-plugin-dev git-hook path/to/plugin.wasm --hook-type post-commit --real-data

# Show debug information
sage-plugin-dev git-hook path/to/plugin.wasm --debug
```
</details>

<details>
<summary><b>🔍 trace</b> - Trace plugin execution</summary>

Shows detailed input/output data during plugin execution.

```bash
sage-plugin-dev trace [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--function <FUNCTION>`: Function to trace (pre-push, post-commit, or run)
- `--branch <BRANCH>`: Branch name for pre-push events [default: main]
- `--commit-id <COMMIT_ID>`: Commit ID for post-commit events [default: abcdef1234567890]
- `--args <ARGS>...`: Arguments for run function

**Examples:**
```bash
# Trace pre-push function execution
sage-plugin-dev trace path/to/plugin.wasm --function pre-push

# Trace post-commit function execution
sage-plugin-dev trace path/to/plugin.wasm --function post-commit --commit-id abc123

# Trace run function execution
sage-plugin-dev trace path/to/plugin.wasm --function run --args arg1 arg2
```
</details>

<details>
<summary><b>⚡ benchmark</b> - Benchmark plugin performance</summary>

Measures plugin performance with statistical analysis.

```bash
sage-plugin-dev benchmark [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--function <FUNCTION>`: Function to benchmark (pre-push, post-commit, or run)
- `--iterations <ITERATIONS>`: Number of iterations to run [default: 100]
- `--branch <BRANCH>`: Branch name for pre-push events [default: main]
- `--commit-id <COMMIT_ID>`: Commit ID for post-commit events [default: abcdef1234567890]
- `--args <ARGS>...`: Arguments for run function

**Examples:**
```bash
# Benchmark pre-push function (100 iterations)
sage-plugin-dev benchmark path/to/plugin.wasm --function pre-push

# Benchmark with custom iterations
sage-plugin-dev benchmark path/to/plugin.wasm --function run --iterations 1000

# Benchmark with custom arguments
sage-plugin-dev benchmark path/to/plugin.wasm --function run --args arg1 arg2
```
</details>

<details>
<summary><b>🧩 mock</b> - Test with mock data</summary>

Tests plugins with predefined mock data scenarios.

```bash
sage-plugin-dev mock [OPTIONS] <PATH>
```

**Arguments:**
- `<PATH>`: Path to the plugin file (.wasm or .js)

**Options:**
- `--function <FUNCTION>`: Function to mock (pre-push, post-commit, or run)
- `--scenario <SCENARIO>`: Mock scenario (normal, empty, long, special-chars, error) [default: normal]
- `--debug`: Show debug information

**Examples:**
```bash
# Test with normal data
sage-plugin-dev mock path/to/plugin.wasm --function pre-push --scenario normal

# Test with empty data
sage-plugin-dev mock path/to/plugin.wasm --function post-commit --scenario empty

# Test with special characters
sage-plugin-dev mock path/to/plugin.wasm --function run --scenario special-chars

# Test with extremely long inputs
sage-plugin-dev mock path/to/plugin.wasm --function pre-push --scenario long
```
</details>

<details>
<summary><b>🛠️ init</b> - Create a new plugin template</summary>

Creates a new plugin project from a template.

```bash
sage-plugin-dev init [OPTIONS] <NAME>
```

**Arguments:**
- `<NAME>`: Name of the plugin

**Options:**
- `--plugin-type <PLUGIN_TYPE>`: Type of plugin to create (rust or js) [default: rust]
- `--dir <DIR>`: Directory to create the plugin in (defaults to current directory)

**Examples:**
```bash
# Create a new Rust plugin
sage-plugin-dev init my-plugin

# Create a new JavaScript plugin
sage-plugin-dev init my-js-plugin --plugin-type js

# Create in a specific directory
sage-plugin-dev init my-plugin --dir ~/projects
```
</details>

## Plugin Development Guide

### Plugin Structure

Each Sage plugin consists of two essential files:

1. **Plugin Binary**:
   - A WebAssembly (`.wasm`) file compiled from Rust, or
   - A JavaScript (`.js`) file

2. **Manifest File**:
   - A JSON (`.json`) file with the same base name as the plugin
   - Contains metadata and function declarations

```
my-plugin.wasm    # Plugin binary (or .js for JavaScript)
my-plugin.json    # Plugin manifest
```

### Plugin Manifest Format

The manifest file defines the plugin's metadata and available functions:

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "functions": ["pre_push", "post_commit", "run"]
}
```

| Field | Description |
|-------|-------------|
| `name` | The name of the plugin (must match the filename without extension) |
| `version` | The plugin version (semantic versioning recommended) |
| `functions` | Array of functions the plugin implements |

### Plugin Functions

Plugins can implement the following functions:

| Function | Description | Event Data |
|----------|-------------|------------|
| `pre_push` | Called before pushing to a remote | Branch name |
| `post_commit` | Called after a commit is created | Commit ID (OID) |
| `run` | Called when the plugin is executed as a CLI command | Command arguments |

### Development Workflow

<div align="center">
  <img src="https://via.placeholder.com/800x200?text=Plugin+Development+Workflow" alt="Plugin Development Workflow" width="800"/>
</div>

#### 1. Create a Plugin

Start by creating a new plugin project:

```bash
sage-plugin-dev init my-plugin
```

#### 2. Implement Plugin Logic

Edit the generated code to implement your plugin's functionality:

- For Rust plugins: Edit `src/lib.rs`
- For JavaScript plugins: Edit `index.js`

#### 3. Build the Plugin

For Rust plugins, build the WebAssembly binary:

```bash
cd my-plugin
cargo build --target wasm32-wasip1 --release
```

#### 4. Test the Plugin

Test each function of your plugin:

```bash
# Test CLI command
sage-plugin-dev run target/wasm32-wasip1/release/my_plugin.wasm

# Test pre-push hook
sage-plugin-dev pre-push target/wasm32-wasip1/release/my_plugin.wasm

# Test post-commit hook
sage-plugin-dev post-commit target/wasm32-wasip1/release/my_plugin.wasm
```

#### 5. Debug and Optimize

Use the advanced commands to debug and optimize your plugin:

```bash
# Trace execution
sage-plugin-dev trace target/wasm32-wasip1/release/my_plugin.wasm --function run

# Benchmark performance
sage-plugin-dev benchmark target/wasm32-wasip1/release/my_plugin.wasm --function run

# Test with edge cases
sage-plugin-dev mock target/wasm32-wasip1/release/my_plugin.wasm --function run --scenario special-chars
```

#### 6. Validate and Install

Validate your plugin and install it:

```bash
# Validate plugin
sage-plugin-dev validate target/wasm32-wasip1/release/my_plugin.wasm

# Install plugin
sage plugin install target/wasm32-wasip1/release/my_plugin.wasm
```

## Command Cheat Sheet

### Basic Commands

| Command | Description | Example |
|---------|-------------|---------|
| `pre-push` | Test pre-push hook | `sage-plugin-dev pre-push plugin.wasm` |
| `post-commit` | Test post-commit hook | `sage-plugin-dev post-commit plugin.wasm` |
| `run` | Test CLI command | `sage-plugin-dev run plugin.wasm arg1 arg2` |
| `info` | Show plugin info | `sage-plugin-dev info plugin.wasm` |
| `validate` | Validate plugin | `sage-plugin-dev validate plugin.wasm` |

### Advanced Commands

| Command | Description | Example |
|---------|-------------|---------|
| `git-hook` | Test with real git data | `sage-plugin-dev git-hook plugin.wasm --real-data` |
| `trace` | Show detailed I/O | `sage-plugin-dev trace plugin.wasm --function run` |
| `benchmark` | Measure performance | `sage-plugin-dev benchmark plugin.wasm --function run` |
| `mock` | Test edge cases | `sage-plugin-dev mock plugin.wasm --function run --scenario special-chars` |

### Common Options

| Option | Description | Example |
|--------|-------------|---------|
| `--verbose` | Show detailed output | `sage-plugin-dev run plugin.wasm --verbose` |
| `--debug` | Show debug information | `sage-plugin-dev pre-push plugin.wasm --debug` |
| `--branch` | Specify branch name | `sage-plugin-dev pre-push plugin.wasm --branch feature/x` |
| `--commit-id` | Specify commit ID | `sage-plugin-dev post-commit plugin.wasm --commit-id abc123` |
| `--function` | Specify function | `sage-plugin-dev trace plugin.wasm --function pre-push` |
| `--iterations` | Set benchmark iterations | `sage-plugin-dev benchmark plugin.wasm --iterations 1000` |
| `--scenario` | Set mock scenario | `sage-plugin-dev mock plugin.wasm --scenario empty` |

## Troubleshooting

<details>
<summary><b>Common Issues</b></summary>

### Plugin Not Found

**Symptom**: Error message "Plugin not found"

**Solution**:
- Ensure the plugin file exists at the specified path
- Check that the plugin name in the manifest matches the filename

### Function Not Supported

**Symptom**: Error message "Plugin does not support X function"

**Solution**:
- Check that the function is declared in the manifest's `functions` array
- Verify that the function is properly exported in your plugin code

### Build Errors

**Symptom**: Compilation errors when building Rust plugins

**Solution**:
- Ensure you have the WASM target installed: `rustup target add wasm32-wasip1`
- Check for syntax errors in your Rust code
- Verify dependencies in Cargo.toml

### Plugin Loading Errors

**Symptom**: Error when loading the plugin

**Solution**:
- Ensure the manifest file exists alongside the plugin binary
- Verify the manifest contains valid JSON
- Check that required fields are present in the manifest
</details>

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the same license as the Sage project.
