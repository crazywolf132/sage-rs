use crate::{gh::pulls, git, tui, ai};
use anyhow::{anyhow, Result};

pub async fn pull_create(
    title: Option<String>,
    body: Option<String>,
    base_branch: Option<String>,
    head_branch: Option<String>,
    draft: bool,
    interactive: bool,
    use_ai: bool,
) -> Result<()> {
    let (owner, repo) = git::repo::owner_repo()?;
    let head_branch = head_branch.unwrap_or(git::branch::current()?);

    // Check to make sure a pull request doesn't already exist
    let pull_request = pulls::get_pr_number(&owner, &repo, &head_branch).await?;
    if pull_request.is_some() {
        println!(
            "Pull request url: http://github.com/{}/{}/pull/{}",
            &owner,
            &repo,
            pull_request.unwrap()
        );
        return Err(anyhow!("A pull request already exists for this branch"));
    }

    // If AI is enabled, use it to generate title and body
    let (title, body, draft) = if use_ai {
        println!("Using AI to generate PR title and body...");
        use crate::git::{commit, branch, repo};
        
        // Get the base branch (default to main if not provided)
        let base = base_branch.clone().or_else(|| repo::default_branch().ok()).unwrap_or_else(|| "main".to_string());
        let current = branch::current()?;
        
        // Get all commits unique to this branch
        let commit_shas = commit::commits_unique_to_current_branch(&base)?;
        
        // Sample commits: first 5, last 5, and every 5th commit in between
        let max_samples = 10;
        let mut sampled_commits = Vec::new();
        
        if commit_shas.len() <= max_samples {
            // If we have fewer commits than our max, use all of them
            sampled_commits = commit_shas;
        } else {
            // Sample first 3, last 2, and every 5th commit in between
            let mut indices = vec![0, 1, 2]; // First 3
            let last_two = commit_shas.len() - 2;
            indices.extend_from_slice(&[last_two, last_two + 1]); // Last 2
            
            // Add every 5th commit in between
            let step = (commit_shas.len() - 5) / 5; // 5 samples in middle
            if step > 0 {
                for i in (3..commit_shas.len() - 2).step_by(step) {
                    indices.push(i);
                }
            }
            
            // Sort indices and get commits
            indices.sort();
            sampled_commits = indices.iter().map(|&i| commit_shas[i].clone()).collect();
        }

        // Generate summaries for sampled commits
        let mut commit_summaries = Vec::new();
        for sha in &sampled_commits {
            let summary = commit::commit_summary(sha)?;
            commit_summaries.push(format!("Commit {}:\n{}", sha, summary));
        }

        // Add summary if we skipped commits
        let mut pr_summary = commit_summaries.join("\n\n---\n\n");
        if commit_shas.len() > sampled_commits.len() {
            let skipped = commit_shas.len() - sampled_commits.len();
            pr_summary.push_str(&format!("\n\n[Note: This PR contains {} commits. Only {} are shown in detail. ", commit_shas.len(), sampled_commits.len()));
            if skipped > 10 {
                pr_summary.push_str(&format!("{} commits are not shown in detail.]", skipped));
            } else {
                pr_summary.push_str(&format!("The remaining {} commits are not shown in detail.]", skipped));
            }
        }

        // Generate PR description prompt
        let ai_title = format!("{}: {} commits", current, commit_shas.len());
        let prompt = ai::prompts::pr_description_prompt(&ai_title, &pr_summary);
        
        // Add warning about truncated content to the PR body
        let ai_body = ai::ask(&prompt).await?;
        let final_body = if commit_shas.len() > sampled_commits.len() {
            format!("{}\n\n---\n\n[Note: This PR contains {} commits. The AI-generated description is based on a sample of {} representative commits. For a complete view of changes, please review the full commit history.]",
                ai_body,
                commit_shas.len(),
                sampled_commits.len())
        } else {
            ai_body
        };

        println!("AI generated title: {}", ai_title);
        (Some(ai_title), Some(final_body), draft)
    }
    // If interactive mode is enabled, use the TUI to get PR details
    else if interactive {
        let details = tui::pull::create_pull_request()?;
        (Some(details.title), Some(details.body), details.draft)
    } else {
        (title, body, draft)
    };

    // Default to "main" for base branch if not provided
    let base_branch = base_branch.or(Some("main".to_string()));

    match pulls::create_pull_request(
        &owner,
        &repo,
        title.as_deref().unwrap_or(""),
        &head_branch,
        base_branch.as_deref().unwrap_or("main"),
        body.as_deref().unwrap_or(""),
        draft,
    )
    .await
    {
        Ok(pr) => {
            println!("Pull request created successfully!");
            println!("Pull request URL: {}", pr.html_url.unwrap());
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to create pull request: {:?}", e)),
    }
}
