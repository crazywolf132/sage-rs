use extism_pdk::*;
use regex::Regex;
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

// Conventional commit types
const VALID_TYPES: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
];

// Post-commit hook to validate commit messages
#[plugin_fn]
pub fn post_commit(input: String) -> FnResult<String> {
    let event: Event = serde_json::from_str(&input)?;
    
    match event {
        Event::PostCommit { oid } => {
            // Get the commit message using the host function
            let commit_msg = get_commit_message(&oid)?;
            
            // Validate the commit message
            match validate_commit_message(&commit_msg) {
                Ok(_) => {
                    let reply = Reply::Ok { 
                        message: format!("Commit {} follows conventional commit format", oid) 
                    };
                    Ok(serde_json::to_string(&reply)?)
                },
                Err(err) => {
                    // We don't want to block the commit, just warn
                    let reply = Reply::Ok { 
                        message: format!("Warning: Commit {} doesn't follow conventional format: {}", oid, err) 
                    };
                    Ok(serde_json::to_string(&reply)?)
                }
            }
        },
        _ => {
            let reply = Reply::Error { 
                message: "Unexpected event type for post_commit function".into() 
            };
            Ok(serde_json::to_string(&reply)?)
        }
    }
}

// CLI command to validate a commit message
#[plugin_fn]
pub fn run(input: String) -> FnResult<String> {
    let cli_args: CliArgs = serde_json::from_str(&input)?;
    
    if cli_args.args.is_empty() {
        let help = format!(
            "Commit Linter Plugin\n\nUsage: sage plugin commit-linter <commit-message>\n\n\
            Validates that commit messages follow the Conventional Commits format:\n\
            <type>[(scope)]: <description>\n\n\
            Valid types: {}\n", 
            VALID_TYPES.join(", ")
        );
        
        let reply = Reply::Ok { message: help };
        return Ok(serde_json::to_string(&reply)?);
    }
    
    let commit_msg = cli_args.args.join(" ");
    match validate_commit_message(&commit_msg) {
        Ok(_) => {
            let reply = Reply::Ok { 
                message: format!("✅ Commit message follows conventional format:\n\n{}", commit_msg) 
            };
            Ok(serde_json::to_string(&reply)?)
        },
        Err(err) => {
            let reply = Reply::Error { 
                message: format!("❌ Invalid commit message: {}\n\n{}", err, commit_msg) 
            };
            Ok(serde_json::to_string(&reply)?)
        }
    }
}

// Helper function to validate a commit message
fn validate_commit_message(message: &str) -> Result<(), String> {
    // Conventional Commits regex: <type>[optional scope]: <description>
    let re = Regex::new(r"^([a-z]+)(\([a-z0-9-]+\))?!?: .+").unwrap();
    
    if !re.is_match(message) {
        return Err("Commit message doesn't match format '<type>[(scope)]: <description>'".into());
    }
    
    // Extract the type
    let captures = re.captures(message).unwrap();
    let commit_type = captures.get(1).unwrap().as_str();
    
    // Check if it's a valid type
    if !VALID_TYPES.contains(&commit_type) {
        return Err(format!(
            "Invalid commit type '{}'. Valid types are: {}", 
            commit_type, 
            VALID_TYPES.join(", ")
        ));
    }
    
    Ok(())
}

// Mock function to get a commit message (in a real plugin, this would call git)
fn get_commit_message(oid: &str) -> FnResult<String> {
    // In a real plugin, we would call git to get the commit message
    // For this example, we'll just return a mock message
    Ok(format!("feat: add new feature (commit {})", oid))
}
