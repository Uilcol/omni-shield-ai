#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use axum::{routing::post, Json, Router};
use serde::Deserialize;

mod comment;
mod github;

#[derive(Deserialize)]
struct Event {
    repository: Repo,
    pull_request: PR,
}

#[derive(Deserialize)]
struct Repo {
    full_name: String,
}

#[derive(Deserialize)]
struct PR {
    number: u64,
}

async fn webhook(Json(event): Json<Event>) -> String {
    let repo = event.repository.full_name;
    let pr = event.pull_request.number;

    // simulação de scan
    let findings = vec!["⚠️ eval() detected", "🔐 hardcoded secret"];

    let report = findings.join("\\n");

    // token fictício (substituir pelo real)
    let token = "GITHUB_TOKEN";

    comment::comment_pr(token, &repo, pr, &report).await;

    "Scan + comment done".into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/webhook/github", post(webhook));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Ultra server running");

    axum::serve(listener, app).await.unwrap();
}
