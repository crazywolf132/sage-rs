mod cmd;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};
use sage_git::ShellGit;

#[derive(Parser)]
#[command(
    author,
    version,
    name = "sage",
    about = "Ai powered stacked-diff git cli tool"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Work(cmd::Work),
    Child(cmd::Child),
    // Restack(cmd::Restack),
    // Commit(cmd::Commit),
    // Stack(cmd::Stack),
    // Undo,
    // Redo,
    // Pr(cmd::Pr),
    Plugin(cmd::Plugin),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo = ShellGit::open("."); // cwd repo
    let mut runtime = cmd::Runtime::init(repo)?; // Shared facade + plugin manager

    match cli.cmd {
        Cmd::Child(c) => c.run(&mut runtime)?,
        Cmd::Work(w) => w.run(&mut runtime)?,
        // Cmd::Restack(r) => r.run(&mut runtime)?,
        // Cmd::Commit(c)  => c.run(&mut runtime)?,
        // Cmd::Stack(s)   => s.run(&mut runtime)?,
        // Cmd::Undo       => runtime.undo()?,
        // Cmd::Redo       => runtime.redo()?,
        // Cmd::Pr(p)      => p.run(&mut runtime)?,
        Cmd::Plugin(p) => p.run(&mut runtime)?,
    }

    Ok(())
}

