/*use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Error as PoiseError, ModelError};

use std::time::Duration;

use serenity::model::id::{UserId, GuildId};

use chrono::Utc;

use crate::db::moderation::muted::Database;

use serenity::model::guild::Member;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "KICK_MEMBERS",
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "user you want to kick?"] user: serenity::User,
    #[description = "duration of the mute?"] number: String,
    #[description = "time unit (s, m, h)"] unit: String,
    #[description = "reason for the kick?"] reason: Option<String>,
) -> Result<(), Error> {


    let duration = match unit.as_str() {
        "s" => {
            let parts = number.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let seconds = parts[0].parse::<u64>().unwrap();
                    Duration::from_secs(seconds)
                }
                _ => {
                    let seconds = parts[0].parse::<u64>().unwrap();
                    Duration::from_secs(seconds)
                }
            }
        }
        "m" => {
            let parts = number.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let seconds = parts[0].parse::<u64>().unwrap();
                    Duration::from_secs(seconds * 60)
                }
                2 => {
                    let minutes = parts[0].parse::<u64>().unwrap();
                    let seconds = parts[1].parse::<u64>().unwrap();

                    Duration::from_secs(minutes * 60 + seconds)
                }
                _ => {
                    let minutes = parts[0].parse::<u64>().unwrap();
                    let seconds = parts[1].parse::<u64>().unwrap();

                    Duration::from_secs(minutes * 60 + seconds)
                }
            }
        }
        "h" => {
            let parts = number.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let hours = parts[0].parse::<u64>().unwrap();
                    Duration::from_secs(hours * 60 * 60)
                }
                2 => {
                    let hours = parts[0].parse::<u64>().unwrap();
                    let minutes = parts[1].parse::<u64>().unwrap();
                    Duration::from_secs(hours * 60 * 60 + minutes * 60)
                }
                3 => {
                    let hours = parts[0].parse::<u64>().unwrap();
                    let minutes = parts[1].parse::<u64>().unwrap();
                    let seconds = parts[2].parse::<u64>().unwrap();
                    Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds)
                }
                _ => {
                    let hours = parts[0].parse::<u64>().unwrap();
                    let minutes = parts[1].parse::<u64>().unwrap();
                    let seconds = parts[2].parse::<u64>().unwrap();
                    Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds)
                }
            }
        }
        _ => {
            ctx.send(|m| {
                m.embed(|e| {
                    e.title("Error")
                        .description("Invalid unit. Only s, m, and h are supported")
                        .color(0xFF0000)
                })
                .ephemeral(true)
            })
            .await?;
            ctx
            return Ok(());
        }
    };

    let current_time_since_epoch = Utc::now().timestamp();

    let duration_in_seconds = duration.as_secs() as i64;

    let timestamp = current_time_since_epoch + duration_in_seconds;


    let member = get_member(&ctx, ctx.guild_id().unwrap(), user.id).await;

    if let Some(member) = member {
        let roles = member.roles;

        // Dereference the role_id to obtain the u64 value
        let mut u64_values: Vec<u64> = Vec::new();
        for role_id in roles {
            u64_values.push(role_id.to_string().parse::<u64>().unwrap());
        }

        println!("u64_values: {:?}", u64_values);

        let db = Database::new("moderation.db").unwrap();

        db.create_table_muted().unwrap();

        db.insert_muted(user.id.into(), ctx.guild_id().unwrap().into(), &reason.unwrap_or("N/a".to_string()), u64_values, timestamp.to_string().as_str()).unwrap();

        
    } else {
       
    }

    Ok(())
}

async fn get_member(ctx: &Context<'_>, guild_id: GuildId, user_id: UserId) -> Option<Member> {
    if let Some(member) = guild_id.member(&ctx, user_id).await.ok() {
        Some(member)
    } else {
        guild_id.member(&ctx, user_id).await.ok()
    }
}*/