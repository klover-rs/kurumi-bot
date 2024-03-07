use reqwest::header::AUTHORIZATION;
use std::fs;

fn get_github_token() -> String {
    let contents = fs::read_to_string("Secrets.toml").unwrap();

    let data: toml::Value = contents.parse().unwrap();

    let discord_token = match data.get("GITHUB_TOKEN") {
        Some(token) => match token.as_str() {
            Some(token_str) => token_str,
            None => panic!("GITHUB_TOKEN value is not a string"),
        },
        None => panic!("GITHUB_TOKEN key not found"),
    };

    println!("GITHUB_TOKEN: {}", discord_token);
    discord_token.to_string()
}

pub async fn fetch_docs(which: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/Asm-Rosie/adhd-helper-bot/main/docs/{}",
        which
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", get_github_token()))
        .send()
        .await?;

    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err("Failed to fetch document".into())
    }
}
