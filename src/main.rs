use sage::{cli::Run, update::check_for_updates};
use clap::Parser;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let _ = check_for_updates().await;

    // Runs the main CLI
    match sage::cli::Cmd::parse().run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

