use reqwest::header::AUTHORIZATION;
use std::fs;

use crate::secrets::get_secret;

fn get_repo() -> String {
    let contents = fs::read_to_string("Secrets.toml").unwrap();

    let data: toml::Value = contents.parse().unwrap();

    let repo = match data.get("GITHUB_REPO") {
        Some(repo) => match repo.as_str() {
            Some(repo_str) => repo_str,
            None => panic!("GITHUB_REPO value is not a string"),
        },
        None => panic!("GITHUB_REPO key not found"),
    };

    println!("GITHUB_REPO: {}", repo);
    repo.to_string()
}

pub async fn fetch_docs(which: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/main/docs/{}",
        get_repo() , which
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", get_secret("GITHUB_TOKEN")))
        .send()
        .await?;

    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err("Failed to fetch document".into())
    }
}
