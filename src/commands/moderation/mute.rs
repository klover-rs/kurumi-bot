use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use serenity::model::id::{GuildId, UserId};

use poise::CreateReply;
use serenity::builder::CreateEmbed;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;

use crate::secrets::get_secret;

use crate::db::moderation::muted::Database;

use crate::commands::components::duration_timer::set_timestamp;

use serenity::model::guild::Member;

fn ephemeral(content: &'static str) -> poise::CreateReply {
    poise::CreateReply::default()
        .content(content)
        .ephemeral(true)
}

async fn parse_duration(
    ctx: &Context<'_>,
    duration: &str,
) -> Result<Option<serenity::Timestamp>, Error> {
    let now = std::time::SystemTime::now();
    //expire_time for minutes
    if duration.ends_with("m") {
        let expire_time = if let Some(duration) = duration.strip_suffix('m') {
            let minutes: u64 = duration.parse()?;
            now + std::time::Duration::from_secs(minutes * 60)
        } else {
            ctx.send(ephemeral("Must be in format {}m or {}h or {}d"))
                .await?;
            return Ok(None);
        };
        let expire_time_secs = expire_time
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        return serenity::Timestamp::from_unix_timestamp(expire_time_secs as i64)
            .map(Some)
            .map_err(Into::into);
    }
    //expire time for hours
    else if duration.ends_with("h") {
        let expire_time = if let Some(duration) = duration.strip_suffix('h') {
            let hours: u64 = duration.parse()?;
            now + std::time::Duration::from_secs(hours * 60 * 60)
        } else {
            ctx.send(ephemeral("Must be in format {}m or {}h or {}d"))
                .await?;
            return Ok(None);
        };
        let expire_time_secs = expire_time
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        return serenity::Timestamp::from_unix_timestamp(expire_time_secs as i64)
            .map(Some)
            .map_err(Into::into);
    }
    // expire time for days
    else if duration.ends_with("d") {
        let expire_time = if let Some(duration) = duration.strip_suffix('d') {
            let days: u64 = duration.parse()?;
            if days > 28 || days == 28 {
                ctx.send(ephemeral("Cant be more than 28 days")).await?;
                return Ok(None);
            }
            now + std::time::Duration::from_secs(days * 60 * 60 * 24)
        } else {
            ctx.send(ephemeral("Must be in format {}m or {}h or {}d"))
                .await?;
            return Ok(None);
        };
        let expire_time_secs = expire_time
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        return serenity::Timestamp::from_unix_timestamp(expire_time_secs as i64)
            .map(Some)
            .map_err(Into::into);
    } else {
        Ok(None)
    }
}

#[poise::command(prefix_command, slash_command, required_permissions = "KICK_MEMBERS")]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::UserId,
    #[description = "How long should the mute last?"] duration: String,
    #[description = "Reason to be shown in audit log"]
    #[rest]
    reason: String,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("This command can only be used in guilds")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let Some(expire_time) = parse_duration(&ctx, &duration).await? else {
        return Ok(());
    };

    let guild_id = ctx.guild_id().unwrap();
    let mut member = ctx.http().get_member(guild_id, user).await?;

    let builder = serenity::EditMember::new()
        .disable_communication_until_datetime(expire_time)
        .audit_log_reason(&reason);

    member.edit(ctx, builder).await?;

    let resp = format!("Timed {} out for `{duration}`.", member.display_name());
    ctx.say(resp).await?;

    let database = Database::new().await.unwrap();

    database.create_table().await.unwrap();

    match database
        .insert(
            user.to_string().parse().unwrap(),
            ctx.guild_id().unwrap().into(),
            &reason,
            duration,
        )
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "KICK_MEMBERS")]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "user you want to unmute?"] user: serenity::UserId,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let mut member = ctx.http().get_member(guild_id, user).await?;

    let builder = serenity::EditMember::new().enable_communication();

    member.edit(ctx, builder).await?;

    ctx.say(format!("Remove timeout from {}", member.display_name()))
        .await?;

    let database = Database::new().await.unwrap();

    match database.read_muted_by_uid(user.into()).await {
        Ok(muted) => {
            if muted.is_empty() {
                ctx.send(
                    CreateReply::default()
                        .content("You cant unmute a member who is not muted")
                        .ephemeral(true),
                )
                .await?;
                return Ok(());
            } else {
                database.delete(user.into()).await.unwrap();

                let roles_vec: Vec<&str> = muted[0].roles.split(',').collect();

                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Member has been unmuted")
                            .description(format!("unmuted <@{}> successfully", user))
                            .color(0x00FF00),
                    ),
                )
                .await?;
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    }

    Ok(())
}
//I removed everything else because the way i did it didnt need em
