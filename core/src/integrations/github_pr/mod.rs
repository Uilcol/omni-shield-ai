use reqwest::blocking::Client;
use serde_json::json;

pub struct GitHubPr;

impl GitHubPr {
    pub fn create_pull_request(
        repo_owner: &str,
        repo_name: &str,
        github_token: &str,
        head_branch: &str,
        base_branch: &str,
        title: &str,
        body: &str,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            repo_owner, repo_name
        );

        let payload = json!({
            "title": title,
            "head": head_branch,
            "base": base_branch,
            "body": body
        });

        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "OmniUil")
            .json(&payload)
            .send()?;

        let status = response.status();
        let text = response.text()?;

        println!("GitHub PR Response: {}", status);
        println!("Response Body: {}", text);

        if !status.is_success() {
            return Err(format!("Failed to create PR: {}", status).into());
        }

        let parsed: serde_json::Value = serde_json::from_str(&text)?;

        Ok(parsed["number"].as_u64().unwrap_or(0))
    }
}
