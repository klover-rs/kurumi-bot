use crate::{Context, Error};

use crate::utils::system_usage;
use reqwest::Client;
use rustc_version::version_meta;
use std::fs;

use crate::download_docs;

#[poise::command(prefix_command, slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let mem_usage = system_usage::memusage()?;
    let version = version_meta()?;

    println!("Rustc version: {}", version.semver);
    println!("Channel: {:?}", version.channel);

    let user_info = fetch_user_info(&"774409449476980746")
        .await
        .expect("Failed to fetch user info");

    let info = download_docs::fetch_docs(&"info.md").await.unwrap();
    println!("{}", &info);

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Info")
                .description(&info)
                .field(
                    "Memory usage <:RAM:1215414863938068620>",
                    format!("{} / {} MB", mem_usage.used_mem, mem_usage.total_mem),
                    true,
                )
                .field(
                    "Rust version <:rust:1215414883072483328>",
                    format!(
                        "Version: `{}`\nChannel: `{:?}`",
                        version.semver, version.channel
                    ),
                    true,
                )
                .author(|a| {
                    a.name(format!("owner: {}", &user_info.username))
                        .icon_url(&user_info.avatar)
                        .url("https://github.com/Asm-Rosie")
                })
                .color(0x672473)
        })
    })
    .await?;

    Ok(())
}

#[derive(Debug)]
struct UserInfo {
    username: String,
    avatar: String,
}
fn get_token() -> String {
    let contents = fs::read_to_string("Secrets.toml").unwrap();

    let data: toml::Value = contents.parse().unwrap();

    let discord_token = match data.get("DISCORD_TOKEN") {
        Some(token) => match token.as_str() {
            Some(token_str) => token_str,
            None => panic!("DISCORD_TOKEN value is not a string"),
        },
        None => panic!("DISCORD_TOKEN key not found"),
    };

    println!("DISCORD_TOKEN: {}", discord_token);
    discord_token.to_string()
}

async fn fetch_user_info(user_id: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let url = format!("https://discord.com/api/v9/users/{}", user_id);
    let client = Client::new();

    let response = client
        .get(&url)
        .header("Authorization", format!("Bot {}", get_token()))
        .send()
        .await?;

    if response.status().is_success() {
        let user_info: serde_json::Value = response.json().await?;
        let username = user_info["username"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let avatar = if let Some(avatar_str) = user_info["avatar"].as_str() {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                user_id, avatar_str
            )
        } else {
            String::new()
        };

        Ok(UserInfo { username, avatar })
    } else {
        println!("Failed to fetch user info.");
        Ok(UserInfo {
            username: String::new(),
            avatar: String::new(),
        })
    }
}
