# Git Stats Plugin

A Sage plugin that provides statistics about your git repository.

## Features

This plugin provides a CLI command to display various statistics about your git repository:

1. Summary of repository statistics
2. Contributor statistics
3. File statistics

## Installing

To install the plugin:

```bash
mkdir -p ~/.config/sage/plugins
cp index.js ~/.config/sage/plugins/git-stats.js
cp git-stats.json ~/.config/sage/plugins/
```

Or use the provided installation script:

```bash
cd ..
./install-js-plugins.sh
```

## Usage

Show all statistics:

```bash
sage plugin git-stats
```

Show a summary:

```bash
sage plugin git-stats summary
```

Show contributor statistics:

```bash
sage plugin git-stats contributors
```

Show file statistics:

```bash
sage plugin git-stats files
```

Show help:

```bash
sage plugin git-stats help
```

## Note

This is a demo plugin that uses mock data. In a real implementation, it would use git commands to gather actual statistics from your repository.
