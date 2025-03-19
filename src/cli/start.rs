use crate::{app, cli::Run};
use clap::Parser;

use anyhow::Result;
use colored::Colorize;


#[derive(Parser, Debug)]
pub struct StartArgs {
    /// The name of the branch to create
    pub name: String,

    /// Optional parent branch to use
    #[clap(short, long)]
    pub parent: Option<String>,
}

impl Run for StartArgs {
    async fn run(&self) -> Result<()> {
        app::start::start(self.name.to_string())?;
        println!("Successfully created branch: {}", self.name.color("green"));
        Ok(())
    }
}