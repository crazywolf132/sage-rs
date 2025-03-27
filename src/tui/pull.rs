use anyhow::Result;

pub struct PullRequestDetails {
    pub title: String,
    pub body: String,
    pub draft: bool,
}

pub fn create_pull_request() -> Result<PullRequestDetails> {
    let title = inquire::Text::new("Title: ").prompt()?;
    let body = inquire::Editor::new("Body: ").prompt()?;
    let draft = inquire::Confirm::new("Draft: ").prompt()?;

    Ok(PullRequestDetails { title, body, draft })
}
