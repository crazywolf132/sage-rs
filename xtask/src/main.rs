use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

/// CI / automation commands for the sage workspace.
///
/// Invoke with `cargo run -p xtask -- <COMMAND>` or create a cargo alias:
/// `[alias]\nxtask = "run -p xtask --"` in your root Cargo.toml.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Xtask {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Format, lint, test - the stuff that must stay green.
    Ci,
    /// Run `cargo fmt --check`.
    Fmt {
        /// Apply fixes instead of just checking.
        #[arg(long)]
        fix: bool,
    },
    /// Run Clippy with `-D warnings`.
    Clippy,
    /// Run `cargo udeps` (requires nightly).
    Udeps,
    /// Run the full test suite.
    Test,
    /// Build release binaries for the current target.
    Build,
    /// Clean + build all targets (cross-compile via `cross`, if installed).
    Release,
}

fn main() -> Result<()> {
    let xtask = Xtask::parse();
    let sh = Shell::new()?;
    match xtask.command {
        Cmd::Ci => ci(&sh)?,
        Cmd::Fmt { fix } => fmt(&sh, fix)?,
        Cmd::Clippy => clippy(&sh)?,
        Cmd::Udeps => udeps(&sh)?,
        Cmd::Test => test(&sh)?,
        Cmd::Build => build(&sh)?,
        Cmd::Release => release(&sh)?,
    }
    Ok(())
}

// -----------------------------------------------------------------------------

fn ci(sh: &Shell) -> Result<()> {
    fmt(sh, false)?;
    clippy(sh)?;
    test(sh)?;
    Ok(())
}

fn fmt(sh: &Shell, fix: bool) -> Result<()> {
    let mode = if fix { "--write" } else { "--check" };
    cmd!(sh, "cargo fmt {mode} --all").run()?;
    Ok(())
}

fn clippy(sh: &Shell) -> Result<()> {
    cmd!(
        sh,
        "cargo clippy --workspace --all-targets --all-features -- -D warnings"
    )
    .run()?;
    Ok(())
}

fn udeps(sh: &Shell) -> Result<()> {
    // nightly / cargo +nightly is required for udeps.
    let nightly = which::which("rustup").is_ok();
    if nightly {
        cmd!(sh, "cargo +nightly udeps --all-targets --all-features").run()?;
    } else {
        eprintln!("\n    ⚠️ cargo-udeps requires a nightly toolchain - skipping");
    }
    Ok(())
}

fn test(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo test --workspace --all-features").run()?;
    Ok(())
}

fn build(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo build --workspace --release").run()?;
    Ok(())
}

fn release(sh: &Shell) -> Result<()> {
    // Prefer `cross` if installed, fallback to plain cargo.
    if which::which("cross").is_ok() {
        cmd!(
            sh,
            "cross build --workspace --release --target x86_64-unknown-linux-musl"
        )
        .run()?;
        cmd!(
            sh,
            "cross build --workspace --release --target aarch64-apple-darwin"
        )
        .run()?;
    } else {
        eprintln!("\n 'cross' not found, building only for host target.\n");
        build(sh)?;
    }
    Ok(())
}
