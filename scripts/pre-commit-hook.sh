#!/bin/bash
# Pre-commit hook to detect and warn about Cargo.toml and Cargo.lock changes mixed with other changes

# Check if Cargo.toml or Cargo.lock is being changed
if git diff --cached --name-only | grep -E 'Cargo\.(toml|lock)$' > /dev/null; then
  # Check if other files are also being changed
  OTHER_CHANGES=$(git diff --cached --name-only | grep -v -E 'Cargo\.(toml|lock)$')
  
  if [ -n "$OTHER_CHANGES" ]; then
    echo "⚠️  WARNING: You're committing changes to Cargo.toml or Cargo.lock along with other files."
    echo "This may lead to merge conflicts. Consider making separate commits for:"
    echo "1. Your feature/bug fix changes"
    echo "2. Dependency or version changes"
    echo ""
    echo "To split your commit:"
    echo "  git reset Cargo.toml Cargo.lock  # Unstage the cargo files"
    echo "  git commit -m \"Your feature commit message\""
    echo "  git add Cargo.toml Cargo.lock"
    echo "  git commit -m \"Update dependencies\""
    echo ""
    echo "To commit anyway, use git commit --no-verify"
    echo ""
    read -p "Do you want to continue with this commit? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      exit 1
    fi
  fi
fi

exit 0
