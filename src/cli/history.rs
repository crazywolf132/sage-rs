use anyhow::{Result};
use clap::Parser;

use crate::{app};

use super::Run;

#[derive(Parser, Debug)]
pub struct History;

impl Run for History {
    async fn run(&self) -> Result<()> {
        app::history::history()
    }
}