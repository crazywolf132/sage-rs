# Version Management Guidelines

To reduce merge conflicts with `Cargo.toml` and `Cargo.lock` files, please follow these guidelines:

## Automated Versioning with CommitSense

As of 10/04/2025, Sage uses [CommitSense](https://github.com/marketplace/actions/commitsense-ai-versioner/) for automated semantic versioning. CommitSense analyzes commit messages using AI to determine appropriate version bumps and generate changelogs.

Benefits of the new system:
- Automatic version determination based on commit content
- AI-powered semantic analysis of commits
- Automated changelog generation
- Reduced manual intervention in the release process

### Conventional Commits (Optional)

While CommitSense can analyze any commit message, using the Conventional Commits format provides more explicit control:
- `feat: ...` - New feature (minor version bump)
- `fix: ...` - Bug fix (patch version bump)
- `docs: ...` - Documentation changes (no version bump)
- `style: ...` - Formatting changes (no version bump)
- `refactor: ...` - Code refactoring (no version bump)
- `perf: ...` - Performance improvement (patch version bump)
- `test: ...` - Adding or fixing tests (no version bump)
- `build: ...` - Build system changes (no version bump)
- `ci: ...` - CI configuration changes (no version bump)
- `chore: ...` - Other changes (no version bump)
- `BREAKING CHANGE: ...` - Breaking API changes (major version bump)
- `feat!: ...` - Feature with breaking changes (major version bump)

## General Rules

1. **Separate commits for version changes**: Always make version bumps in separate commits from feature work.
2. **Use the provided script**: Use the `scripts/update_version.sh` script for manual version updates.
3. **Coordinate version changes**: Communicate with the team before updating versions manually.
4. **Pull before pushing**: Always pull the latest changes before pushing version updates.

## For Developers

- Avoid manually editing the version in `Cargo.toml`
- If you need to add a new dependency, make it a separate commit from your feature work
- Run `cargo update` in a separate commit if needed
- When resolving merge conflicts in `Cargo.lock`, prefer using `git checkout --theirs Cargo.lock` and then running `cargo update` locally

## For CI/CD

- The CI pipeline has been updated to not commit lockfile changes during nightly releases
- Regular releases will still commit both files as needed

## Using the Version Update Script

```bash
# To update to a specific version
./scripts/update_version.sh 0.2.11

# For pre-release versions
./scripts/update_version.sh 0.2.11-beta.1
```

This script will update both `Cargo.toml` and `Cargo.lock` files appropriately.

## Git Configuration

We've added a `.gitattributes` file that uses the union merge strategy for `Cargo.lock` and `Cargo.toml`, which should help reduce conflicts. No additional configuration is needed on your part.
