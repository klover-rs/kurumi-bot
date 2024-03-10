use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Error as PoiseError, ModelError};

use serenity::model::id::{UserId, GuildId};

use serenity::model::guild::Member;

use poise::CreateReply;
use serenity::builder::CreateEmbed;

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


    let member = get_member(&ctx, ctx.guild_id().unwrap(), user.id).await;

    match member {
        Some(member) => {
            match member.kick(&ctx).await {
                Ok(_) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Kicked")
                            .description(format!(
                                "kicked {}\nreason: {}", user.name, &reason.unwrap_or("not provided.".to_string())
                            )) 
                    )).await?; 
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