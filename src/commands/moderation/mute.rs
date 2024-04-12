use crate::commands::moderation::punishment::{self, PunishmentType};
use crate::{Context, Error};
use poise::serenity_prelude::model::user;
use poise::serenity_prelude::{self as serenity, EditRole};


use serenity::model::id::{UserId, GuildId};

use serenity::builder::CreateEmbed;
use poise::CreateReply;

use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;

use crate::secrets::get_secret;

use crate::db::moderation::muted::Database;

use crate::commands::components::duration_timer::set_timestamp;

use serenity::model::guild::Member;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "KICK_MEMBERS",
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "user you want to kick?"] user: serenity::User,
    #[description = "duration of the mute?"] duration: String,
    #[description = "time unit (s, m, h)"] unit: String,
    #[description = "reason for the mute?"] reason: Option<String>,
) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.send(CreateReply::default().content("This command can only be used in guilds").ephemeral(true)).await?;
            return Ok(());
        }
    };

    
    let timestamp = match set_timestamp(ctx, unit, duration).await {
        Ok(timestamp) => {
            if timestamp == 0 {
                return Ok(());
            } else {
                timestamp
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let member = get_member(&ctx, guild_id, user.id).await;

    let muter = get_member(&ctx, guild_id, ctx.author().id).await;

    let top_role_muter = muter.clone().unwrap().highest_role_info(ctx);
    let top_role_member = member.clone().unwrap().highest_role_info(ctx);

    let guild = guild_id.to_partial_guild(ctx).await?;

    let role = match guild.role_by_name("muted") {
        Some(role) => role,
        None => {

            let reply = {
                let components = vec![
                    serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("yes")
                            .style(serenity::ButtonStyle::Success)
                            .label("yes"),
                        serenity::CreateButton::new("no")
                            .style(serenity::ButtonStyle::Danger)
                            .label("no"),
                    ])
                ];
                CreateReply::default().content("Role 'muted' not found, do you want to create it?").components(components)
            };

            let msg = ctx.send(reply).await?;   

            

            while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.clone())
                .author_id(ctx.author().id)
                .channel_id(ctx.channel_id())
                .timeout(std::time::Duration::from_secs(120))
                .await
            {
                match mci.data.custom_id.as_str() {
                    "yes" => {
                        let new_role = guild.create_role(ctx, EditRole::default().name("muted").permissions(serenity::Permissions::empty())).await?;

                        msg.edit(ctx, CreateReply::default().content(format!("created role <@&{}> successfully", new_role.id)).components(vec![])).await?;
                    }
                    "no" => {
                        msg.edit(ctx, CreateReply::default().content("operation cancelled").components(vec![])).await?;
                    }
                    _ => {}
                }

                mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                break;
            };

            return Ok(());
        }
    };

    println!("role: {:?}", role);
    
    

    match (top_role_muter, top_role_member) {
        (Some(top_role_muter), Some(top_role_member)) => {
            if top_role_muter.1 > top_role_member.1 {
                println!("you can mute this user");
                

                mute_member(&ctx, &role.id, &member.unwrap(), guild_id.try_into()?, timestamp, reason.clone()).await?;
                let time_now = chrono::Utc::now();
        
                ctx.send(CreateReply::default().embed(
                    CreateEmbed::default()
                    .title("Member has been muted")
                    .description(format!("muted <@{}> successfully", user.id))
                    .fields(vec![
                        ("User", format!("<@{}>", user.id), true),
                        ("Reason", format!("{}", &reason.unwrap_or("no reason".to_string())), true),
                        ("Muted until", format!("<t:{}:R>", timestamp), true), 
                    ])
                    .timestamp(time_now)
                    .color(0xFF0000)
            
                )).await?;
            } else {
                println!("you cant mute this user");
                ctx.send(CreateReply::default().content("you cant mute this user, because they have a higher role than you").ephemeral(true)).await?;
            }
        }
        (Some(_top_role_muter), None) => {
            println!("you can mute this user");
            mute_member(&ctx, &role.id, &member.unwrap(), guild_id.try_into()?, timestamp, reason.clone()).await?;
            let time_now = chrono::Utc::now();
        
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                .title("Member has been muted")
                .description(format!("muted <@{}> successfully", user.id))
                .fields(vec![
                    ("User", format!("<@{}>", user.id), true),
                    ("Reason", format!("{}", &reason.unwrap_or("no reason".to_string())), true),
                    ("Muted until", format!("<t:{}:R>", timestamp), true), 
                ])
                .timestamp(time_now)
                .color(0xFF0000)
        
            )).await?;

        }
        _ => {
            ctx.send(CreateReply::default().content("you cant mute this user, because you have no roles, read the docs for more informations.").ephemeral(true)).await?;
        } 
    }

    Ok(())
}

async fn mute_member(ctx: &Context<'_>, muted_role: &serenity::RoleId, member: &serenity::Member, guild_id: i64, duration: i64, reason: Option<String>) -> Result<(), Error> {
    let roles = &member.roles;

    let mut u64_roles: Vec<u64> = Vec::new();

    for role_id in roles {
        u64_roles.push(role_id.to_string().parse::<u64>().unwrap());
    }

    println!("u64_values: {:?}", u64_roles);

    let db = Database::new().await?;
    db.create_table().await?;

    let user_id = member.user.id.to_string().parse::<i64>().unwrap();
    
    let reason_str = reason.clone().unwrap_or("N/a".to_string());

    db.insert(user_id, guild_id, &reason_str, u64_roles, duration).await?;

    manage_roles(None, user_id, guild_id).await?;

    member.add_role(ctx, muted_role).await?;

    let punishment = punishment::Punishment {
        user_id: user_id,
        reason: reason,
        punishment_type: PunishmentType::Mute,
        moderator_id: ctx.author().id.try_into().unwrap(),
        guild_id: guild_id.try_into().unwrap(),
        duration: Some(duration),
        delete_messages: None
    };

    punishment::send_to_mod_log_channel(*ctx, &punishment).await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "KICK_MEMBERS",
)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "user you want to unmute?"] user: serenity::User,
) -> Result<(), Error> {

    match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            ctx.send(CreateReply::default().content("This command can only be used in guilds").ephemeral(true)).await?;
            return Ok(());
        }
    };

    

    let database = Database::new().await.unwrap();

    match database.read_muted_by_uid(user.id.into()).await {
        Ok(muted) => {
            if muted.is_empty() {
                ctx.send(CreateReply::default().content("You cant unmute a member who is not muted").ephemeral(true)).await?;
                return Ok(());
            } else {
                database.delete(user.id.into()).await.unwrap();

                let roles_vec: Vec<&str> = muted[0].roles.split(',').collect();

                manage_roles(Some(roles_vec), user.id.into(), ctx.guild_id().unwrap().into()).await.unwrap();
                ctx.send(CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Member has been unmuted")
                        .description(format!("unmuted <@{}> successfully", user.id))
                        .color(0x00FF00)
                        
                )).await?;

            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
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

async fn manage_roles(roles: Option<Vec<&str>>, uid: i64, guild_id: i64) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, uid);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let token = get_secret("DISCORD_TOKEN");

    let mut authorization_value = HeaderValue::from_str(&format!("Bot {}", token)).unwrap();
    authorization_value.set_sensitive(true);
    headers.insert(AUTHORIZATION, authorization_value);

    let roles = match roles {
        Some(roles) => roles,
        None => {
            [].to_vec()
        }
    };
    

    let body = json!({
        "roles": roles
    });

    let response = client
    .patch(&url)
    .headers(headers)
    .json(&body)
    .send()
    .await?;

    println!("Response: {:?}", response);
    Ok(())


}
