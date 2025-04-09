# Version Management Guidelines

To reduce merge conflicts with `Cargo.toml` and `Cargo.lock` files, please follow these guidelines:

## General Rules

1. **Separate commits for version changes**: Always make version bumps in separate commits from feature work.
2. **Use the provided script**: Use the `scripts/update_version.sh` script for version updates.
3. **Coordinate version changes**: Communicate with the team before updating versions.
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
