![Sage Banner](docs/sage.jpg)

> **Note**: This is a temporary repository for the Rust remake of Sage. For the original Golang version, visit [https://github.com/crazywolf132/sage](https://github.com/crazywolf132/sage).

Hey there! Welcome to Sage - your friendly neighborhood Git companion. Think of it as a smart wrapper around Git that helps you streamline your workflow.

## Why Did I Build This? 🤔

Let's be real - Git is powerful, but sometimes its workflow can be streamlined. I built this because:
- I wanted to automate repetitive Git workflows
- Switching contexts between terminal and browser for PR management was tedious
- Merge conflicts were taking up too much of my day
- I knew there had to be a faster way to handle common Git tasks

So I built Sage to make my life easier, and hopefully yours too!

## What Makes Sage Cool? ✨

* **Workflow Automation**: Sage handles common Git operations with smart defaults and built-in best practices.
* **Simple Commands**: Instead of typing `git checkout -b feature/branch && git pull origin main && git push -u origin feature/branch`, just do `sage start feature/branch`. Your fingers will thank you.
* **Time Travel (Kind of)**: A super detailed undo system that lets you track and revert operations with precision.
* **PR Magic**: Create and manage pull requests right from your terminal. No more context-switching to GitHub!
* **Branch Wizardry**: Smart syncing with automatic stash handling and conflict detection.
* **AI Helper** 🤖: Optional AI features for commit messages and PR content (needs OpenAI API key).

## Getting Started 🚀

### Requirements
- Go 1.20 or later (required for module support)
- Git (obviously!)
- GitHub account for PR features

### Installation Options

1. Quick Install (recommended):
```bash
cargo install sage-rs
```

2. Install directly from GitHub:
```bash
cargo install --git https://github.com/crazywolf132/sage-rs.git
```

3. Build from source:
```bash
git clone https://github.com/crazywolf132/sage-rs.git
cd sage-rs
cargo build --release
./target/release/sage -v
```

4. Verify installation:
```bash
sage --help
sage -v
```

## Basic Usage 🛠️

### Start a new branch
```bash
sage start feature/awesome-stuff
```
Boom! New branch created, latest updates pulled, and pushed to GitHub. All in one go.

### Commit your masterpiece
```bash
sage commit "Add that thing that does the stuff"
```
Stages and commits everything. No more `git add .` followed by `git commit -m` dance.

### Push it real good
```bash
sage push
```
Pushes your work to origin. If you need --force, Sage will make sure you don't shoot yourself in the foot.

### Oops! (Undo System) 🔄
```bash
# See what you've been up to
sage undo --history

# Take back that last thing you did
sage undo

# Undo something specific
sage undo --id <operation-id>

# Preview before you undo (safety first!)
sage undo --preview

# Find exactly what you need to undo
sage undo --category commit --group branch
```

### PR stuff made easy
```bash
# Create a PR
sage pr create --title "🚀 Add awesome feature" --body "Trust me, this is good"

# See what's cooking
sage pr list

# Check out someone's PR
sage pr checkout 42

# See what reviewers are saying
sage pr todos 42

# Merge it in
sage pr merge 42 --method squash
```

## Setting Things Up ⚙️

### Environment Variables
- **`SAGE_GITHUB_TOKEN`** or **`GITHUB_TOKEN`**: Your GitHub token for API access
  - Required scopes: `repo`, `read:org` (for organization repos)
  - See [GitHub Integration Guide](src/gh/README.md) for detailed setup instructions
  - If you have the `gh` CLI installed and authenticated, Sage will automatically use that token!
- `SAGE_CONFIG`: Where to keep your config file
- `SAGE_OPENAI_KEY`: For AI features (totally optional)

### GitHub Authentication
If you encounter GitHub API errors like `Error: Github`, you need to set up authentication:

```bash
# Option 1: Set environment variable (add to your shell profile)
export SAGE_GITHUB_TOKEN=your_github_token

# Option 2: Use GitHub CLI 
gh auth login  # Sage will use this automatically
```

For a complete guide on GitHub authentication, see the [GitHub Integration Guide](src/gh/README.md).

### Quick Config
```bash
# AI Settings
sage config set ai.model gpt-4        # AI model to use
sage config set ai.base_url <url>     # Custom AI API endpoint (optional)

# Git Settings
sage config set git.default_branch main    # Default branch for operations
sage config set git.merge_method squash    # Default PR merge method

# PR Settings
sage config set pr.draft false            # Create PRs as drafts by default
sage config set pr.reviewers user1,user2  # Default PR reviewers
sage config set pr.labels feature,docs    # Default PR labels
```

### Experimental Features 🧪
Sage includes experimental features that can enhance your Git workflow. View and manage them with:
```bash
sage config experimental
```

Available experimental features:
- **Reuse Recorded Resolution (rerere)**: Git remembers how you resolved conflicts and automatically reuses those resolutions.
  ```bash
  # Enable globally (all Sage repos)
  sage config set experimental.rerere true
  # Enable for current repo only
  sage config set --local experimental.rerere true
  ```

- **Commit Graph**: Speeds up git log operations in large repositories by maintaining a commit graph.
  ```bash
  sage config set experimental.commit-graph true
  ```

- **File System Monitor**: Significantly improves `git status` performance by using OS-level file monitoring.
  ```bash
  sage config set experimental.fsmonitor true
  ```

- **Git Auto-Maintenance**: Automatically optimizes repository performance with scheduled maintenance tasks:
  - Hourly prefetch to keep your repo up-to-date
  - Automatic loose object cleanup
  - Daily reference packing
  - Incremental repack for optimal storage
  ```