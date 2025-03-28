use anyhow::Result;
use crate::{app, cli::Run};
use clap::Parser;

/// Command to display the current git repository status
///
/// Shows a comprehensive view of your repository's current state including
/// branch information, staged and unstaged changes, and untracked files.
#[derive(Parser, Debug)]
#[clap(after_help = "STATUS SYMBOLS:
  A - Added file (new file staged for commit)
  M - Modified file (changes staged for commit)
  D - Deleted file (file deletion staged for commit)
  R - Renamed file (file rename staged for commit)
  C - Copied file (file copy staged for commit)
  ? - Untracked file (new file not yet staged)
  ! - Ignored file (file matching a pattern in .gitignore)

COMBINED STATUS SYMBOLS:
  AM - Added to index, modified in working tree
  AD - Added to index, deleted in working tree
  MM - Modified in index, modified in working tree
  MD - Modified in index, deleted in working tree
  
BRANCH INDICATORS:
  ↑n - n commits ahead of remote
  ↓n - n commits behind remote
  $ - Stashed changes exist")]
pub struct StatusArgs;

impl Run for StatusArgs {
    async fn run(&self) -> Result<()> {
        app::status::status()?;
        Ok(())
    }
}