use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use poise::serenity_prelude::{Error as PoiseError, ModelError};

use serenity::model::id::{UserId, GuildId};

use poise::CreateReply;

use crate::commands::moderation::punishment;

use serenity::model::guild::Member;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "user you want to ban?"] user: serenity::User,
    #[description = "reason for the ban?"] reason: Option<String>,
    #[description = "remove message from the last 7 days?"] delete_messages: Option<bool>,
) -> Result<(), Error> {

    let delete_message_days = match delete_messages {
        Some(true) => {
            7
        }
        Some(false) => {
            0
        }
        None => {
            0
        }
    };

    let member = get_member(&ctx, ctx.guild_id().unwrap(), user.id).await;

    match member {
        Some(member) => {
            match member.ban(&ctx, delete_message_days).await {
                Ok(_) => {
                    ctx.say(format!("banned {}\nreason: {}", user.name, reason.clone().unwrap_or("not provided.".to_string()))).await?;

                    let punishment = punishment::Punishment {
                        user_id: user.id.try_into().unwrap(),
                        reason: reason,
                        punishment_type: punishment::PunishmentType::Ban,
                        moderator_id: ctx.author().id.try_into().unwrap(),
                        guild_id: ctx.guild_id().unwrap().try_into().unwrap(),
                        duration: None,
                        delete_messages: delete_messages
                    };
                    punishment::send_to_mod_log_channel(ctx, &punishment).await?;
                }
                Err(PoiseError::Model(ModelError::GuildNotFound)) => {
                    ctx.say("Member not found").await?;
                }
                Err(PoiseError::Model(ModelError::InvalidPermissions { required, present })) => {
                    ctx.send(CreateReply::default().content(format!("Missing permissions: {}\npresent permissions: {}", required, present)).ephemeral(true)).await?;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
            
        }
        None => {
            ctx.say("Member not found").await?;
        }
    }

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "userID you want to unban?"] user: String,
) -> Result<(), Error>{

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.send(CreateReply::default().content("This command can only be used in guilds").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let user_id = match user {
        s if s.parse::<u64>().is_ok() => {
            s.parse::<u64>().unwrap()
        }
        _ => {
            ctx.say("Not a integer").await?;
            return Ok(())
        }
    };

    match guild_id.unban(&ctx, UserId::from(user_id)).await {
        Ok(_) => {
            ctx.say(format!("unbanned {}", &user_id)).await?;
        }
        Err(PoiseError::Model(ModelError::GuildNotFound)) => {
            ctx.say("Member not found").await?;
        }
        Err(PoiseError::Model(ModelError::InvalidPermissions { required, present })) => {
            ctx.send(CreateReply::default().content(format!("Missing permissions: {}\npresent permissions: {}", required, present)).ephemeral(true)).await?;
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    Ok(())
}

async fn get_member(ctx: &Context<'_>, guild_id: GuildId, user_id: UserId) -> Option<Member> {
    if let Some(member) = guild_id.member(&ctx, user_id).await.ok() {
        Some(member)
    } else {
        guild_id.member(&ctx, user_id).await.ok()
    }
}
