#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use reqwest::Client;

pub async fn comment_pr(token: &str, repo: &str, pr: u64, msg: &str) {
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr
    );

    let client = Client::new();

    let _ = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "omniuil")
        .json(&serde_json::json!({ "body": msg }))
        .send()
        .await;
}
