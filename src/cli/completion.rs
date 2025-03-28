use crate::cli::Run;
use anyhow::Result;
use clap::{Parser, CommandFactory};
use clap_complete::{generate, shells::Bash, shells::Zsh, shells::Fish};
use std::fmt;
use std::io;

#[derive(Parser, Debug)]
#[clap(after_help = "INSTALLATION INSTRUCTIONS:

Bash:
  $ sage completion bash > ~/.bash_completion.d/sage
  # Make sure the directory exists first:
  # mkdir -p ~/.bash_completion.d
  # Add to ~/.bashrc if not already sourcing completion directory:
  # source ~/.bash_completion.d/sage

Zsh:
  $ mkdir -p ~/.zsh/completions
  $ sage completion zsh > ~/.zsh/completions/_sage
  # Add to ~/.zshrc if not already in fpath:
  # fpath=(~/.zsh/completions $fpath)
  # autoload -U compinit && compinit

Fish:
  $ sage completion fish > ~/.config/fish/completions/sage.fish")]
pub struct CompletionArgs {
    /// Shell to generate completions for
    #[clap(value_enum, long_help = "Specifies which shell's completion script to generate. Choose from:
- bash: Generates Bash completions that work with bash-completion
- zsh: Generates Zsh completions using the _describe completer
- fish: Generates Fish shell completions

The generated script will be output to stdout, which you can redirect to the appropriate location for your shell.")]
    pub shell: Shell,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
pub enum Shell {
    /// Bash shell completions (compatible with bash-completion)
    Bash,
    /// Zsh shell completions (using the _describe completer)
    Zsh,
    /// Fish shell completions
    Fish,
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "Bash"),
            Shell::Zsh => write!(f, "Zsh"),
            Shell::Fish => write!(f, "Fish"),
        }
    }
}

impl Run for CompletionArgs {
    async fn run(&self) -> Result<()> {
        let mut cmd = crate::cli::Cmd::command();
        let mut stdout = io::stdout();

        // Print a helpful comment at the top of the generated script
        match self.shell {
            Shell::Bash => {
                println!("# Bash completion script for sage");
                println!("# Save this output to ~/.bash_completion.d/sage");
                println!("# Make sure the directory exists: mkdir -p ~/.bash_completion.d/");
                println!("# Add to ~/.bashrc: source ~/.bash_completion.d/sage");
                println!("#");
                generate(Bash, &mut cmd, "sage", &mut stdout);
            }
            Shell::Zsh => {
                println!("# Zsh completion script for sage");
                println!("# Save this output to ~/.zsh/completions/_sage");
                println!("# Make sure the directory exists: mkdir -p ~/.zsh/completions/");
                println!("# Add to ~/.zshrc:");
                println!("# fpath=(~/.zsh/completions $fpath)");
                println!("# autoload -U compinit && compinit");
                println!("#");
                generate(Zsh, &mut cmd, "sage", &mut stdout);
            }
            Shell::Fish => {
                println!("# Fish completion script for sage");
                println!("# Save this output to ~/.config/fish/completions/sage.fish");
                println!("# Make sure the directory exists: mkdir -p ~/.config/fish/completions/");
                println!("#");
                generate(Fish, &mut cmd, "sage", &mut stdout);
            }
        }

        // Print a helpful message to stderr after generating the script
        println!("\n# Generated completion script for {} shell", self.shell.to_string().to_lowercase());
        println!("# See installation instructions with: sage completion --help");

        Ok(())
    }
}

// Simplified value validation for branch names - used by the CLI argument parser only
pub mod value_completion {
    use crate::git::branch;
    
    pub fn branch_names(value: &str) -> Result<String, String> {
        // Try to get branch names
        match branch::list() {
            Ok(branches) => {
                // If value is empty or matches beginning of a branch, it's valid
                if value.is_empty() || branches.iter().any(|b| b.starts_with(value)) {
                    return Ok(value.to_string());
                }
                
                // If it doesn't match any branch prefix, but it's explicitly provided, accept it
                // (let users create new branches by explicitly typing them)
                if !value.is_empty() {
                    return Ok(value.to_string());
                }
                
                // For better error messages
                Err(format!("Invalid branch name: {}. Available branches: {}", 
                            value, 
                            branches.join(", ")))
            },
            Err(_) => {
                // If we couldn't get branch list, just accept the input
                // (This happens outside a git repo)
                Ok(value.to_string())
            }
        }
    }
} 