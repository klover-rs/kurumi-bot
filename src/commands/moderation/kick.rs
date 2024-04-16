use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Error as PoiseError, ModelError};

use serenity::model::id::{UserId, GuildId};

use serenity::model::guild::Member;

use poise::CreateReply;
use serenity::builder::CreateEmbed;

use crate::commands::moderation::punishment::{self, *};

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "KICK_MEMBERS",
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "user you want to kick?"] user: serenity::User,
    #[description = "reason for the kick?"] reason: Option<String>,
) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.send(CreateReply::default().content("This command can only be used in guilds").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let member = get_member(&ctx, guild_id, user.id).await;

    match member {
        Some(member) => {
            match member.kick(&ctx).await {
                Ok(_) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Kicked")
                            .description(format!(
                                "kicked {}\nreason: {}", user.name, &reason.clone().unwrap_or("not provided.".to_string())
                            )) 
                    )).await?; 

                    
                    let punishment = punishment::Punishment {
                        user_id: user.id.try_into().unwrap(),
                        reason: reason,
                        punishment_type: PunishmentType::Kick,
                        moderator_id: ctx.author().id.try_into().unwrap(),
                        guild_id: guild_id.try_into().unwrap(),
                        duration: None,
                        delete_messages: None
                    };
                    punishment::send_to_mod_log_channel(ctx, &punishment).await?;
                }
                Err(PoiseError::Model(ModelError::GuildNotFound)) => {
                    ctx.send(CreateReply::default().content("Member not found").ephemeral(true)).await?;
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
            ctx.send(CreateReply::default().content("Member not found").ephemeral(true)).await?;
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