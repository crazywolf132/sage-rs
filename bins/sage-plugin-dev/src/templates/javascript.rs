/// Template for index.js
pub fn index_js_template(name: &str) -> String {
    format!(
        r#"// {} Plugin

// Define the CLI args structure
function parseCliArgs(input) {{
  return JSON.parse(input);
}}

// Define the event types
const EventType = {{
  PRE_PUSH: 'pre_push',
  POST_COMMIT: 'post_commit'
}};

// Define the reply types
function createOkReply(message) {{
  return JSON.stringify({{
    kind: 'ok',
    message: message
  }});
}}

function createErrorReply(message) {{
  return JSON.stringify({{
    kind: 'error',
    message: message
  }});
}}

// Pre-push hook
function prePush(input) {{
  const event = JSON.parse(input);

  if (event.event !== 'pre_push') {{
    return createErrorReply('Unexpected event type for pre_push function');
  }}

  const {{ branch }} = event;

  // Your validation logic here
  return createOkReply(`Pre-push check passed for branch: ${{branch}}`);
}}

// Post-commit hook
function postCommit(input) {{
  const event = JSON.parse(input);

  if (event.event !== 'post_commit') {{
    return createErrorReply('Unexpected event type for post_commit function');
  }}

  const {{ oid }} = event;

  // Your post-commit logic here
  return createOkReply(`Post-commit check passed for commit: ${{oid}}`);
}}

// CLI command
function run(input) {{
  const cliArgs = parseCliArgs(input);

  if (cliArgs.args.length === 0) {{
    return createOkReply(`Hello from \${name}! This is a Sage plugin.\n\nUsage: sage plugin \${name} [args...]`);
  }}

  return createOkReply(`Hello from \${name}! Args: ${{JSON.stringify(cliArgs.args)}}`);
}}

// Register plugin functions
Plugin.registerFunction('pre_push', prePush);
Plugin.registerFunction('post_commit', postCommit);
Plugin.registerFunction('run', run);
"#,
        name
    )
}

/// Template for manifest.json
pub fn manifest_json_template(name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "functions": ["pre_push", "post_commit", "run"]
}}"#,
        name
    )
}

/// Template for README.md
pub fn readme_md_template(name: &str) -> String {
    format!(
        r#"# {} Plugin

A JavaScript plugin for Sage.

## Features

This plugin demonstrates:

1. Handling pre-push hooks
2. Handling post-commit hooks
3. Implementing a CLI command

## Installing

To install the plugin:

```bash
mkdir -p ~/.config/sage/plugins
cp index.js ~/.config/sage/plugins/{}.js
cp {}.json ~/.config/sage/plugins/{}.json
```

## Usage

### CLI Command

Run the plugin as a CLI command:

```bash
sage plugin {}
```

With arguments:

```bash
sage plugin {} arg1 arg2
```

### Git Hooks

The plugin will automatically:

1. Run pre-push checks when pushing to a remote
2. Run post-commit checks when creating a commit
"#,
        name, name, name, name, name, name
    )
}

/// Template for .gitignore
pub fn gitignore_template() -> String {
    r#"node_modules/
.DS_Store
"#.to_string()
}
