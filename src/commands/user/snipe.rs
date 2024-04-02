use crate::secrets;
use crate::{Context, Error};

use poise::serenity_prelude as serenity;

use chrono::Utc;
use poise::CreateReply;
use reqwest::Client;
use serenity::builder::CreateEmbed;

use crate::db::moderation::logs::DatabaseMsgLogs;
///Snipe the last deleted message
#[poise::command(prefix_command, slash_command)]
pub async fn snipe(ctx: Context<'_>) -> Result<(), Error> {
    let db = DatabaseMsgLogs::new().await?;

    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default()
                    .content("action is not involved with a guild (likely a dm)")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let last_deleted_msg = db.get_last_deleted_msgs(guild_id).await?;

    let user_info = fetch_user_info(&last_deleted_msg[0].author_id.to_string())
        .await
        .expect("Failed to fetch user info");

    let current_time = Utc::now();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title(format!("sniped {}'s message!", user_info.username))
                .thumbnail(user_info.avatar)
                .fields([
                    ("Content", last_deleted_msg[0].contents.clone(), false),
                    (
                        "Author",
                        format!("<@{}>", last_deleted_msg[0].author_id.to_string()),
                        true,
                    ),
                    (
                        "Channel",
                        format!("<#{}>", last_deleted_msg[0].channel_id.to_string()),
                        true,
                    ),
                ])
                .timestamp(current_time)
                .color(0xa33a0d),
        ),
    )
    .await?;

    Ok(())
}

#[derive(Debug)]
struct UserInfo {
    username: String,
    avatar: String,
}

async fn fetch_user_info(user_id: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let url = format!("https://discord.com/api/v9/users/{}", user_id);
    let client = Client::new();

    let response = client
        .get(&url)
        .header(
            "Authorization",
            format!("Bot {}", secrets::get_secret("DISCORD_TOKEN")),
        )
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
