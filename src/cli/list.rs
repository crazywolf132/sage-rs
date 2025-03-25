use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug)]
pub struct ListArgs;

impl Run for ListArgs {
    async fn run(&self) -> Result<()> {
        app::list::list()?;
        Ok(())
    }
}