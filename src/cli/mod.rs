pub use crate::cli::cmd::*;

use anyhow::Result;

use crate::update;
pub mod clone;
mod cmd;
pub mod commit;
pub mod start;
pub mod status;
pub mod push;
pub mod switch;
pub mod list;
pub mod completion;
pub mod pr;
pub mod sync;
pub mod nuke;
pub mod clean;
pub mod history;

pub trait Run {
    async fn run(&self) -> Result<()>;
}

impl Run for Cmd {
    async fn run(&self) -> Result<()> {
        // Check for updates before running any command
        if let Err(e) = update::check_for_updates().await {
            eprintln!("Warning: Failed to check for updates: {}", e);
        }

        match self {
            Cmd::Commit(cmd) => cmd.run().await,
            Cmd::Clone(cmd) => cmd.run().await,
            Cmd::Start(cmd) => cmd.run().await,
            Cmd::Status(cmd) => cmd.run().await,
            Cmd::Push(cmd) => cmd.run().await,
            Cmd::Switch(cmd) => cmd.run().await,
            Cmd::List(cmd) => cmd.run().await,
            Cmd::Completion(cmd) => cmd.run().await,
            Cmd::Pr(cmd) => cmd.run().await,
            Cmd::Sync(cmd) => cmd.run().await,
            Cmd::Nuke(cmd) => cmd.run().await,
            Cmd::Clean(cmd) => cmd.run().await,
            Cmd::History(cmd) => cmd.run().await,
        }
    }
}
