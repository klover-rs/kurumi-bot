


use poise::serenity_prelude::{ChannelId, CreateEmbed, CreateMessage};

use crate::{Context, Error};
use std::fmt;

pub enum PunishmentType {
    Ban,
    Kick,
    Mute
}

impl fmt::Display for PunishmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PunishmentType::Ban => write!(f, "Ban"),
            PunishmentType::Kick => write!(f, "Kick"),
            PunishmentType::Mute => write!(f, "Mute"),
        }
    }
}

pub struct Punishment {
    pub punishment_type: PunishmentType,
    pub reason: Option<String>,
    pub delete_messages: Option<bool>,
    pub duration: Option<i64>,
    pub user_id: i64,
    pub guild_id: i64,
    pub moderator_id: i64
}

use crate::db::configuration::Database;

pub async fn send_to_mod_log_channel(ctx: Context<'_>, punishment: &Punishment) -> Result<(), Error> {

    let db = Database::new().await.unwrap();
    db.create_table().await.unwrap();

    let config = db.read_by_guild_id(punishment.guild_id).await.unwrap();
    if config.is_empty() {
        return Ok(());
    }

    let mod_log_channel = config[0].mod_log_channel;

    if mod_log_channel == 0 {
        return Ok(());
    }

    let fields = match punishment.punishment_type {
        PunishmentType::Ban => {
            vec![
                ("User", format!("<@{}>", punishment.user_id), true),
                ("Moderator", format!("<@{}>", punishment.moderator_id), true),
                ("Reason", format!("{}", punishment.reason.clone().unwrap_or("n/a".to_string())), true),
                ("Deleted messages", format!("{}", punishment.delete_messages.unwrap_or(false)), true),
            ]
        },
        PunishmentType::Kick => {
            vec![
                ("User", format!("<@{}>", punishment.user_id), true),
                ("Moderator", format!("<@{}>", punishment.moderator_id), true),
                ("Reason", format!("{}", punishment.reason.clone().unwrap_or("n/a".to_string())), true),
            ]
        }, 
        PunishmentType::Mute => {
            vec![
                ("User", format!("<@{}>", punishment.user_id), true),
                ("Moderator", format!("<@{}>", punishment.moderator_id), true),
                ("Reason", format!("{}", punishment.reason.clone().unwrap_or("n/a".to_string())), true),
                ("Duration", format!("<t:{}:R>", punishment.duration.unwrap_or(0)), true),
            ]
        }
    };
    

    let channel_id = ChannelId::from(mod_log_channel as u64);

    channel_id.send_message(ctx, CreateMessage::default()
        .add_embed(
            CreateEmbed::default()
                .title(format!("{}", punishment.punishment_type))
                .fields(fields)
        )).await?;

    Ok(())

}