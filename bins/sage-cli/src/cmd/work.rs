use crate::cmd::Runtime;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Work {
    pub name: String,
}

impl Work {
    pub fn run(self, rt: &mut Runtime) -> Result<()> {
        Ok(())
    }
}
