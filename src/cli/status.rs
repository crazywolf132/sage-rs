use anyhow::Result;
use crate::{app, cli::Run};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct StatusArgs;


impl Run for StatusArgs {
    async fn run(&self) -> Result<()> {
        app::status::status()?;
        Ok(())
    }
}