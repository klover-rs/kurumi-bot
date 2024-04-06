use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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

    let reason = reason.unwrap_or("N/a".to_string());

    
    
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

    if let Some(member) = member {
        let roles = &member.roles;

        
        let mut u64_roles: Vec<u64> = Vec::new();
        for role_id in roles {
            u64_roles.push(role_id.to_string().parse::<u64>().unwrap());
        }

        println!("u64_values: {:?}", u64_roles);

       

        let database = Database::new().await.unwrap();

        database.create_table().await.unwrap();

        match database.insert(user.id.to_string().parse().unwrap(), ctx.guild_id().unwrap().into(), &reason, u64_roles, timestamp).await {
            Ok(_) => {
                ()
            },
            Err(e) => {
                println!("Error: {:?}", e);
                return Ok(());
            }
        }

        //None is in this case removing ALL ROLES, SO BE CAREFUL HOW YOU USE THIS FUNCTION AND ITS ARGUMENTS PLEASE :3
        manage_roles(None, user.id.into(), ctx.guild_id().unwrap().into()).await.unwrap();

        let time_now = chrono::Utc::now();
        
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default()
            .title("Member has been muted")
            .description(format!("muted <@{}> successfully", user.id))
            .fields(vec![
                ("User", format!("<@{}>", user.id), true),
                ("Reason", format!("{}", &reason), true),
                ("Muted until", format!("<t:{}:R>", timestamp), true), 
            ])
            .timestamp(time_now)
            .color(0xFF0000)
            
        )).await?;
        
    } else {
        println!("Member not found");
    }

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