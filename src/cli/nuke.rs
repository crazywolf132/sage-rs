use anyhow::Result;
use clap::Parser;

use crate::app;

use super::Run;

#[derive(Parser, Debug)]
pub struct NukeArgs {}

impl Run for NukeArgs {
    async fn run(&self) -> Result<()> {
        app::nuke::nuke().await
    }
}
