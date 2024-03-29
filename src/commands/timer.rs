use crate::{Context, Error};

use crate::db::timer::Database;

use std::time::Duration;

use crate::download_docs;

use poise::serenity_prelude as serenity;

use poise::CreateReply;
use serenity::builder::CreateEmbed;

use crate::commands::components::duration_timer;

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("set", "list", "delete", "help")
)]
pub async fn timer(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("no subcommand has been called uwu").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "what is the timer for?"] description: String,
    #[description = "what is the unit of the timer? (only s, m, and h are supported)"] unit: String,
    #[description = "what is the duration of the timer? e.g. (if m, 10, 50, 20:40, 10:10)"] number: String,
) -> Result<(), Error> {
    let timestamp = match duration_timer::set_timestamp(ctx, unit, number).await {
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

    let database = Database::new().await.unwrap();

    database.create_table().await.unwrap();

    let user_id = ctx.author().id;
    let dm_channel = ctx.author().create_dm_channel(&ctx).await?;

    println!("user_id: {}", user_id);
    println!("dm_channel: {}", dm_channel.id);

    let msg = ctx
        .send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Timer has been set")
                    .description("Please make sure to have your **DMS enabled for this server!**")
                    .field("Description", &description, true)
                    .field("Time", &format!("<t:{}:R>", timestamp), true)
                    .color(0x00FF00),
            ),
        )
        .await?;

    database
        .insert(
            user_id.to_string().parse::<i64>().unwrap(),
            &description,
            timestamp,
            dm_channel.id.to_string().parse::<i64>().unwrap(),
        )
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(10)).await;

    msg.delete(ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let database = Database::new().await.unwrap();

    database.create_table().await.unwrap();

    match database
        .read_by_uid(ctx.author().id.to_string().parse::<i64>().unwrap())
        .await
    {
        Ok(data) => {
            if data.is_empty() {
                ctx.say("No timers found").await?;
                return Ok(());
            }
            let mut counter = 0;
            let mut list_string = String::new();

            for timer_record in data {
                counter += 1;

                list_string.push_str(&format!(
                    "{}. id: {} | description: {} | time: <t:{}:R>\n",
                    counter, timer_record.id, timer_record.description, timer_record.duration
                ));
            }

            println!("{}", list_string);

            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Timer list")
                        .description(format!(
                            "all of your timers are listed below: \n------------------\n{}",
                            &list_string
                        )),
                ),
            )
            .await?;
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "what is the id of the timer you want to delete?"] data_id: i32,
) -> Result<(), Error> {
    let database = Database::new().await.unwrap();

    database.create_table().await.unwrap();

    match database
        .read_by_uid(ctx.author().id.to_string().parse::<i64>().unwrap())
        .await
    {
        Ok(data) => {
            if data.is_empty() {
                ctx.say("No timers found").await?;
            }

            let mut found = false;

            for timer_record in data {
                if timer_record.id == data_id {
                    database.delete_by_id(timer_record.id).await.unwrap();
                    found = true;
                    break;
                } else if timer_record.id > data_id {
                    continue;
                } else if timer_record.id < data_id {
                    continue;
                } else {
                    continue;
                }
            }

            if !found {
                ctx.say("Timer not found").await?;
            } else {
                ctx.say("Timer deleted").await?;
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let result = download_docs::fetch_docs(&"commands/timer.md")
        .await
        .unwrap();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Help")
                .description(format!("{}", result))
                .color(0x0000FF),
        ),
    )
    .await?;

    println!("{}", result);

    Ok(())
}
