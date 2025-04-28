//! helpers to parse porcelain-safe git output.

use crate::error::*;

pub fn commit_list(raw: &str) -> Result<Vec<(String, String, String, String, String)>> {
    raw.lines()
        .map(|l| {
            let parts: Vec<&str> = l.split('|').collect();
            if parts.len() < 5 {
                return Err(GitError::Parse("expected '<sha>|<subject>|<author>|<timestamp>|<body>'".to_string()));
            }

            let id = parts[0].to_owned();
            let subject = parts[1].trim().to_owned();
            let author = parts[2].trim().to_owned();
            let timestamp = parts[3].trim().to_owned();
            let body = parts[4..].join("|").trim().to_owned(); // Join any remaining parts as body

            Ok((id, subject, author, timestamp, body))
        })
        .collect()
}