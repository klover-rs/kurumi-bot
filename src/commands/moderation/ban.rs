use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Error as PoiseError, ModelError};

use serenity::model::id::{UserId, GuildId};

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
                    ctx.say(format!("banned {}\nreason: {}", user.name, reason.unwrap_or("not provided.".to_string()))).await?;
                }
                Err(PoiseError::Model(ModelError::GuildNotFound)) => {
                    ctx.say("Member not found").await?;
                }
                Err(PoiseError::Model(ModelError::InvalidPermissions(missing_perms))) => {
                    ctx.say(format!("Missing permissions: {:?}", missing_perms)).await?;
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
    #[description = "user you want to unban?"] user: serenity::User,
) -> Result<(), Error>{
    let member = get_member(&ctx, ctx.guild_id().unwrap(), user.id).await;

    match member {
        Some(member) => {
            match member.unban(&ctx).await {
                Ok(_) => {
                    ctx.say(format!("unbanned {}", user.name)).await?;  
                }
                Err(PoiseError::Model(ModelError::GuildNotFound)) => {
                    ctx.say("Member not found").await?;
                }
                Err(PoiseError::Model(ModelError::InvalidPermissions(missing_perms))) => {
                    ctx.say(format!("Missing permissions: {:?}", missing_perms)).await?;
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

async fn get_member(ctx: &Context<'_>, guild_id: GuildId, user_id: UserId) -> Option<Member> {
    if let Some(member) = guild_id.member(&ctx, user_id).await.ok() {
        Some(member)
    } else {
        guild_id.member(&ctx, user_id).await.ok()
    }
}