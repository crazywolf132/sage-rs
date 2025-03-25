use sage::cli::Run;
use clap::Parser;
use std::process::ExitCode;
use tiny_update_notifier::check_github;

#[tokio::main]
async fn main() -> ExitCode {
    // Checks for version updates 
    check_github(
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_REPOSITORY")
    );

    // Runs the main CLI
    match sage::cli::Cmd::parse().run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

