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
    #[description = "reason for the kick?"] reason: Option<String>,
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

        remove_all_roles(user.id.into(), ctx.guild_id().unwrap().into()).await.unwrap();

        
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default()
            .title("Member has been muted")
            .description(format!("muted <@{}> successfully", user.id))
            .fields(vec![
                ("User", format!("<@{}>", user.id), true),
                ("Reason", format!("{}", &reason), true),
                
            ])
            .color(0xFF0000)
            
        )).await?;
        
    } else {
        println!("Member not found");
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

async fn remove_all_roles( uid: i64, guild_id: i64) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, uid);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let token = get_secret("DISCORD_TOKEN");

    let mut authorization_value = HeaderValue::from_str(&format!("Bot {}", token)).unwrap();
    authorization_value.set_sensitive(true);
    headers.insert(AUTHORIZATION, authorization_value);

    let body = json!({
        "roles": []
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