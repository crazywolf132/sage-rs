use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SyncArgs;

impl SyncArgs {
    pub async fn run(&self) -> Result<()> {
        todo!()
    }
}