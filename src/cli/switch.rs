use crate::{app, cli::Run};
use clap::Parser;
use anyhow::Result;
use crate::cli::completion::value_completion;

#[derive(Parser, Debug)]
pub struct SwitchArgs {
    /// The name of the branch to switch to
    #[clap(value_parser = value_completion::branch_names)]
    pub name: Option<String>,
}

impl Run for SwitchArgs {
    async fn run(&self) -> Result<()> {
        app::switch::switch(self.name.clone().unwrap_or("main".to_string()))?;
        Ok(())
    }
}