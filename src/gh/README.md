# GitHub Integration Guide

This module provides GitHub API integration for Sage using the `octocrab` crate. 

## Authentication

Sage provides multiple methods to authenticate with GitHub, in the following priority order:

1. **Environment Variables**:
   - `SAGE_GITHUB_TOKEN`: Preferred method specific to Sage
   - `GITHUB_TOKEN`: Standard GitHub token environment variable

2. **GitHub CLI**: 
   - If you have the GitHub CLI (`gh`) installed and authenticated, Sage will automatically use your GitHub token

3. **Git Credential Helper**:
   - Falls back to using your git configuration's credential helper

## Setting Up Authentication

### Option 1: Use Environment Variables (Recommended)

1. Create a GitHub personal access token:
   - Go to [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens)
   - Click "Generate new token"
   - Select scopes: `repo`, `read:org` (for organization repos)
   - Copy the generated token

2. Add the token to your environment:
   ```bash
   # Add to your shell profile (.bashrc, .zshrc, etc.)
   export SAGE_GITHUB_TOKEN=your_token_here
   ```

### Option 2: Use the GitHub CLI

1. Install the GitHub CLI: 
   - Follow instructions at [https://cli.github.com/](https://cli.github.com/)

2. Authenticate with GitHub:
   ```bash
   gh auth login
   ```

3. Sage will automatically detect and use the token from the GitHub CLI

## Troubleshooting

If you see the error `Error: Github` or authentication errors:

1. Check that your token has the correct scopes (`repo`, `read:org`)
2. Verify the token is correctly set in your environment: `echo $SAGE_GITHUB_TOKEN`
3. Try running `gh auth status` to check your GitHub CLI authentication
4. For debugging, you can run Sage with extra verbosity: `RUST_LOG=debug sage <command>`

## Creating a GitHub Token

1. Go to [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens)
2. Click "Generate new token" (select "classic" token)
3. Add a descriptive note like "Sage CLI"
4. Select the following scopes:
   - `repo` (all)
   - `read:org` (if working with organization repositories)
5. Click "Generate token"
6. Copy the token and set it as described above

Remember: Keep your token secure and never commit it to version control. 