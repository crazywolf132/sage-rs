use anyhow::Result;
use clap::Parser;

use crate::app;

use super::Run;

#[derive(Parser, Debug)]
pub struct CleanArgs {}

impl Run for CleanArgs {
    async fn run(&self) -> Result<()> {
        app::clean::clean().await
    }
}