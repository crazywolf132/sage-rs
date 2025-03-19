pub use crate::cli::cmd::*;

use anyhow::Result;

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

pub trait Run {
    async fn run(&self) -> Result<()>;
}

impl Run for Cmd {
    async fn run(&self) -> Result<()> {
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
        }
    }
}
