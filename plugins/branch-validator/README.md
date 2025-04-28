# Branch Validator Plugin

A Sage plugin that validates branch names according to common naming conventions.

## Features

This plugin:

1. Validates branch names before pushing to a remote
2. Provides a CLI command to validate branch names manually
3. Ensures branches follow standard naming patterns

## Valid Branch Patterns

The following branch naming patterns are supported:

- `feature/<feature-name>`: For new features
- `bugfix/<bug-name>`: For bug fixes
- `hotfix/<fix-name>`: For urgent fixes
- `release/v<version>`: For release branches (e.g., release/v1.0.0)
- `docs/<doc-name>`: For documentation changes
- `refactor/<name>`: For code refactoring
- `test/<test-name>`: For test-related changes
- `chore/<chore-name>`: For maintenance tasks
- `main`: Main branch
- `develop`: Development branch

## Installing

To install the plugin:

```bash
mkdir -p ~/.config/sage/plugins
cp index.js ~/.config/sage/plugins/branch-validator.js
cp branch-validator.json ~/.config/sage/plugins/
```

Or use the provided installation script:

```bash
cd ..
./install-js-plugins.sh
```

## Usage

### CLI Command

Validate a branch name:

```bash
sage plugin branch-validator "feature/new-feature"
```

Show help:

```bash
sage plugin branch-validator
```

### Git Hooks

The plugin automatically validates branch names before pushing to a remote. If a branch doesn't follow the naming convention, the push is blocked with an error message.
