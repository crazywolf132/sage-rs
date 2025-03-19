use clap::Parser;
use anyhow::Result;
use crate::ai;

use super::Run;

#[derive(Parser, Debug)]
pub struct HiArgs {
    prompt: String,
}

impl Run for HiArgs {
    async fn run(&self) -> Result<()> {
        let response = ai::ask(&self.prompt).await?;
        println!("{}", response);
        Ok(())
    }
}