use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

pub struct GitHubReviewClient;

impl GitHubReviewClient {
    pub fn create_pr_review_comment(
        repo_owner: &str,
        repo_name: &str,
        pull_number: u64,
        github_token: &str,
        body_text: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            repo_owner, repo_name, pull_number
        );

        let body = json!({
            "body": body_text
        });

        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "OmniUil")
            .json(&body)
            .send()?;

        let status = response.status();
        let response_text = response.text()?;

        println!("GitHub PR Review Response: {}", status);
        println!("Response Body: {}", response_text);

        if !status.is_success() {
            return Err(format!("GitHub PR Review request failed: {}", status).into());
        }

        Ok(())
    }
}
