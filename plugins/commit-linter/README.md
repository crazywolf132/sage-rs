# Commit Linter Plugin

A Sage plugin that validates commit messages according to the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## Features

This plugin:

1. Validates commit messages after they are created
2. Provides a CLI command to validate commit messages manually
3. Ensures commits follow the format: `<type>[(scope)]: <description>`

## Valid Commit Types

The following commit types are supported:

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Changes that don't affect code meaning (formatting, etc.)
- `refactor`: Code changes that neither fix bugs nor add features
- `perf`: Performance improvements
- `test`: Adding or fixing tests
- `build`: Changes to build system or dependencies
- `ci`: Changes to CI configuration
- `chore`: Other changes that don't modify src or test files

## Building

To build this plugin:

```bash
cd plugins/commit-linter
rustup target add wasm32-wasip1  # Only needed once
cargo build --target wasm32-wasip1 --release
```

This will create a WASM file at `target/wasm32-wasip1/release/commit_linter_plugin.wasm`.

## Installing

To install the plugin:

```bash
mkdir -p ~/.config/sage/plugins
cp target/wasm32-wasip1/release/commit_linter_plugin.wasm ~/.config/sage/plugins/commit-linter.wasm
cp commit-linter.json ~/.config/sage/plugins/commit-linter.json
```

Or use the Sage plugin install command:

```bash
sage plugin install target/wasm32-wasip1/release/commit_linter_plugin.wasm
```

## Usage

### CLI Command

Validate a commit message:

```bash
sage plugin commit-linter "feat: add new feature"
```

Show help:

```bash
sage plugin commit-linter
```

### Git Hooks

The plugin automatically validates commit messages after they are created. If a commit doesn't follow the conventional format, a warning is displayed but the commit is still allowed.
