use anyhow::Result;

pub mod branch;
pub mod pull;

pub use branch::*;

pub use pull::*; 


// confirm
pub fn confirm(message: &str) -> Result<bool> {
    let selection = inquire::Confirm::new(message).prompt()?;
    Ok(selection)
}