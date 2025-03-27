use anyhow::Result;
use clap::Parser;

use crate::{app, git};

#[derive(Parser, Debug)]
pub struct SyncArgs;

impl SyncArgs {
    pub async fn run(&self) -> Result<()> {
        match app::sync::sync() {
            Ok(_) => Ok(()),
            Err(_) => {
                // if there was an error doing this, we will try and give the user their changes back
                // so as not to break their work.
                if git::stash::has_stash()? {
                    git::stash::apply_stash()?;
                }
                Ok(())
            }
        }
    }
}