use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

pub struct GitHubApiClient;

impl GitHubApiClient {
    pub fn create_commit_status(
        repo_owner: &str,
        repo_name: &str,
        commit_sha: &str,
        github_token: &str,
        description: &str,
        state: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/statuses/{}",
            repo_owner, repo_name, commit_sha
        );

        let body = json!({
            "state": state,
            "target_url": "https://github.com",
            "description": description,
            "context": "OmniUil Security Gate"
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

        println!("GitHub API Response: {}", status);
        println!("Response Body: {}", response_text);

        if !status.is_success() {
            return Err(format!("GitHub API request failed: {}", status).into());
        }

        Ok(())
    }
}
