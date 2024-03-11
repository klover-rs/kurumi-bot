use poise::FrameworkContext;

use poise::serenity_prelude as serenity;

use crate::{
    Data,
    Error,
    db::moderation::logs::Database,
    secrets::get_secret
};

use serenity::builder::CreateMessage;
use serenity::builder::CreateEmbed;
use serenity::builder::CreateEmbedFooter;
use chrono::{Utc, DateTime};

use poise::serenity_prelude::all::Message;

use poise::serenity_prelude::all::{MessageId, ChannelId};

pub async fn handle_messages(message: &Message , _framework: FrameworkContext<'_, Data, Error>) -> Result<(), Error> {

    if message.author.bot { return Ok(()); }

    let msg_id = message.id.to_string().parse::<u64>().unwrap();
    let guild_id = message.guild_id.unwrap().to_string().parse::<u64>().unwrap();
    let channel_id = message.channel_id.to_string().parse::<u64>().unwrap();
    let author_id = message.author.id.to_string().parse::<u64>().unwrap();
    let content = message.content.clone();
    let attachments = message.attachments.clone();

    let mut urls: Vec<String> = Vec::new();
    
    for attachment in attachments {
        urls.push(attachment.url.clone());
    }

    println!("attachments: {:?}", urls);

    println!("{}: {}\nmessage id: {}", message.author.name, message.content, message.id.to_string());

    let db = Database::new("moderation_logs.db")?;

    db.create_table_logs()?;

    db.insert_log(msg_id, guild_id, channel_id, author_id, &content)?;

    Ok(())
}

pub async fn deleted_messages_handler(message_id: &MessageId, ctx: &serenity::Context) -> Result<(), Error> {
    println!("deleted message: {}", message_id);

    let db = Database::new("moderation_logs.db")?;

    let message: Vec<(u64, u64, u64, u64, String)> = db.read_logs_by_id(message_id.to_string().parse().unwrap())?;

    if message.is_empty() {
        println!("message not found");
        return Ok(());
    } else if message[0].3 == get_secret("BOT_ID").parse::<u64>().unwrap() {
        println!("bot message");
        return Ok(());
    }

    let log_channel = get_secret("LOG_CHANNEL").parse::<u64>().unwrap();

    let channel_id = ChannelId::from(log_channel);

    channel_id.send_message(&ctx.http, CreateMessage::default().add_embed(
        CreateEmbed::default()
            .title("Deleted message")
            .description(format!("{}", message[0].4))
            .field("Channel", format!("<#{}>", message[0].2), false)
            .field("Author", format!("<@{}>", message[0].3), false)
            .footer(CreateEmbedFooter::new(format!("msg id: {}", message[0].0)))
            .color(0xFF0000)
            
    )).await?;

    Ok(())
}

pub async fn edited_messages_handler(message_id: &MessageId, new_message: &str, ctx: &serenity::Context) -> Result<(), Error> {
    
    let db = Database::new("moderation_logs.db")?;

    let message: Vec<(u64, u64, u64, u64, String)> = db.read_logs_by_id(message_id.to_string().parse().unwrap())?;

    if message.is_empty() {
        println!("message not found");
        return Ok(());
    } else if message[0].3 == get_secret("BOT_ID").parse::<u64>().unwrap() {
        println!("bot message");
        return Ok(());
    }

    let log_channel = get_secret("LOG_CHANNEL").parse::<u64>().unwrap();

    let channel_id = ChannelId::from(log_channel);


    let current_timestamp: DateTime<Utc> = Utc::now();

    channel_id.send_message(&ctx.http, CreateMessage::default().add_embed(
        CreateEmbed::default()
            .title("Edited message")
            .description(format!("**new message**:\n{}\n**old message**:\n{}", new_message, message[0].4))
            .field("Channel", format!("<#{}>", message[0].2), false)
            .field("Author", format!("<@{}>", message[0].3), false)
            .footer(CreateEmbedFooter::new(format!("msg id: {}", message[0].0)))
            .timestamp(current_timestamp)
            .color(0x00FF00)
    )).await?;

    db.update_log_content(message_id.to_string().parse().unwrap(), &new_message)?; 

    Ok(())

}
