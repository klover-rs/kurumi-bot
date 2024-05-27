use poise::serenity_prelude::CacheHttp;
use poise::FrameworkContext;

use poise::serenity_prelude as serenity;

use crate::{db::moderation::logs::DatabaseMsgLogs, secrets::get_secret, Data, Error};

use crate::db::configuration::Database as DatabaseConfig;

use chrono::{DateTime, Utc};
use serenity::builder::CreateEmbed;
use serenity::builder::CreateEmbedFooter;
use serenity::builder::CreateMessage;

use poise::serenity_prelude::all::Message;

use poise::serenity_prelude::all::{ChannelId, MessageId};

pub async fn handle_messages(
    message: &Message,
    _framework: FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    if message.author.bot {
        return Ok(());
    }

    let msg_id = message.id.to_string().parse::<i64>().unwrap();
    let guild_id = match message.guild_id {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            println!("action is not involved with a guild (likely a dm)");
            return Ok(());
        }
    };
    let channel_id = message.channel_id.to_string().parse::<i64>().unwrap();
    let author_id = message.author.id.to_string().parse::<i64>().unwrap();
    let content = message.content.clone();
    let attachments = message.attachments.clone();

    let mut urls: Vec<String> = Vec::new();

    for attachment in attachments {
        urls.push(attachment.url.clone());
    }

    println!("attachments: {:?}", urls);

    println!(
        "{}: {}\nmessage id: {}",
        message.author.name, message.content, message.id
    );

    let db = DatabaseMsgLogs::new().await?;

    db.create_table_msg_logs().await?;

    db.insert_msg_logs(msg_id, guild_id, channel_id, author_id, &content, urls)
        .await?;

    Ok(())
}

pub async fn deleted_messages_handler(
    channel_id: &ChannelId,
    message_id: &MessageId,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    println!("..");

    let guild_id = match channel_id.to_channel(ctx.http()).await {
        Ok(channel) => match channel.guild() {
            Some(guild_channel) => guild_channel.guild_id.to_string().parse::<i64>().unwrap(),
            None => {
                println!("action is not involved with a guild (likely a dm)");
                return Ok(());
            }
        },
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        }
    };

    let config_db = DatabaseConfig::new().await?;
    config_db.create_table().await?;
    let config = match config_db.read_by_guild_id(guild_id).await {
        Ok(config) => {
            if config.is_empty() {
                println!("config not found\n--------------------------------");
                return Ok(());
            }
            config
        }
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        }
    };

    println!(
        "deleted message: {}\n--------------------------------",
        message_id
    );

    let db = DatabaseMsgLogs::new().await?;

    let message = db
        .read_logs_by_id(message_id.to_string().parse().unwrap(), guild_id)
        .await?;

    if message.is_empty() {
        println!("message not found\n--------------------------------");
        return Ok(());
    } else if message[0].author_id == get_secret("BOT_ID").parse::<i64>().unwrap() {
        println!("bot message\n--------------------------------");
        return Ok(());
    }

    let log_channel = config[0].log_channel;

    println!("log channel: {}", log_channel);

    let channel_id = ChannelId::from(log_channel as u64);

    channel_id
        .send_message(
            &ctx.http,
            CreateMessage::default().add_embed(
                CreateEmbed::default()
                    .title("Deleted message")
                    .description(format!(
                        "`{}`\n{}",
                        message[0].contents,
                        message[0]
                            .attachments
                            .split(", ")
                            .collect::<Vec<&str>>()
                            .join("\n")
                    ))
                    .field("Channel", format!("<#{}>", message[0].channel_id), false)
                    .field("Author", format!("<@{}>", message[0].author_id), false)
                    .footer(CreateEmbedFooter::new(format!(
                        "msg id: {}",
                        message[0].msg_id
                    )))
                    .color(0xFF0000),
            ),
        )
        .await?;

    db.create_table_deleted_msgs().await?;

    let attachment_vec: Vec<String> = message[0]
        .attachments
        .split(", ")
        .map(|s| s.to_owned()) // or: .map(String::from)
        .collect();

    db.insert_deleted_msgs(
        message[0].msg_id,
        message[0].guild_id,
        message[0].channel_id,
        message[0].author_id,
        &message[0].contents,
        attachment_vec,
    )
    .await?;

    Ok(())
}

pub async fn edited_messages_handler(
    channel_id: &ChannelId,
    message_id: &MessageId,
    new_message: &str,
    ctx: &serenity::Context,
) -> Result<(), Error> {
    let guild_id = match channel_id.to_channel(ctx.http()).await {
        Ok(channel) => match channel.guild() {
            Some(guild_channel) => guild_channel.guild_id.to_string().parse::<i64>().unwrap(),
            None => {
                println!("action is not involved with a guild (likely a dm)");
                return Ok(());
            }
        },
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        }
    };

    let config_db = DatabaseConfig::new().await?;
    config_db.create_table().await?;
    let config = match config_db.read_by_guild_id(guild_id).await {
        Ok(config) => {
            if config.is_empty() {
                println!("config not found\n--------------------------------");
                return Ok(());
            }
            config
        }
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        }
    };

    let db = DatabaseMsgLogs::new().await?;

    let message = db
        .read_logs_by_id(message_id.to_string().parse().unwrap(), guild_id)
        .await?;

    if message.is_empty() {
        println!("message not found\n--------------------------------");
        return Ok(());
    } else if message[0].author_id == get_secret("BOT_ID").parse::<i64>().unwrap() {
        println!("bot message\n--------------------------------");
        return Ok(());
    }

    let log_channel = config[0].log_channel;

    let channel_id = ChannelId::from(log_channel as u64);

    let current_timestamp: DateTime<Utc> = Utc::now();

    channel_id
        .send_message(
            &ctx.http,
            CreateMessage::default().add_embed(
                CreateEmbed::default()
                    .title("Edited message")
                    .description(format!(
                        "**new message**:\n`{}`\n**old message**:\n`{}`",
                        new_message, message[0].contents
                    ))
                    .field("Channel", format!("<#{}>", message[0].channel_id), false)
                    .field("Author", format!("<@{}>", message[0].author_id), false)
                    .footer(CreateEmbedFooter::new(format!(
                        "msg id: {}",
                        message[0].msg_id
                    )))
                    .timestamp(current_timestamp)
                    .color(0x0787F7),
            ),
        )
        .await?;

    db.update_logs_by_id(message_id.to_string().parse().unwrap(), new_message)
        .await?;

    Ok(())
}
