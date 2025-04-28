use extism_pdk::*;
use serde::{Deserialize, Serialize};

// Define the event structure that matches the host's Event enum
#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum Event {
    PrePush { branch: String },
    PostCommit { oid: String },
}

// Define the reply structure that matches the host's Reply enum
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Reply {
    Ok { message: String },
    Error { message: String },
}

// Define the CLI args structure
#[derive(Deserialize)]
struct CliArgs {
    args: Vec<String>,
}

// Pre-push hook
#[plugin_fn]
pub fn pre_push(input: String) -> FnResult<String> {
    let event: Event = serde_json::from_str(&input)?;
    
    match event {
        Event::PrePush { branch } => {
            // Example validation: don't allow pushing to main directly
            if branch == "main" {
                let reply = Reply::Error { 
                    message: "Direct pushes to main branch are not allowed. Please create a PR instead.".into() 
                };
                return Ok(serde_json::to_string(&reply)?);
            }
            
            let reply = Reply::Ok { 
                message: format!("Pre-push check passed for branch: {}", branch) 
            };
            Ok(serde_json::to_string(&reply)?)
        },
        _ => {
            let reply = Reply::Error { 
                message: "Unexpected event type for pre_push function".into() 
            };
            Ok(serde_json::to_string(&reply)?)
        }
    }
}

// Post-commit hook
#[plugin_fn]
pub fn post_commit(input: String) -> FnResult<String> {
    let event: Event = serde_json::from_str(&input)?;
    
    match event {
        Event::PostCommit { oid } => {
            // Just log the commit, no validation needed
            let reply = Reply::Ok { 
                message: format!("Commit {} was processed", oid) 
            };
            Ok(serde_json::to_string(&reply)?)
        },
        _ => {
            let reply = Reply::Error { 
                message: "Unexpected event type for post_commit function".into() 
            };
            Ok(serde_json::to_string(&reply)?)
        }
    }
}

// CLI command
#[plugin_fn]
pub fn run(input: String) -> FnResult<String> {
    let cli_args: CliArgs = serde_json::from_str(&input)?;
    
    let message = if cli_args.args.is_empty() {
        "Hello, World! This is a Sage plugin.\n\nUsage: sage plugin hello-world [name]\n".to_string()
    } else {
        format!("Hello, {}! Welcome to Sage plugins.\n", cli_args.args[0])
    };
    
    let reply = Reply::Ok { message };
    Ok(serde_json::to_string(&reply)?)
}
