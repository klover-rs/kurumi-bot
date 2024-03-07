use crate::{Context, Error};
use chrono::Utc;

use crate::db::timer::Database;

use std::time::Duration;

use crate::download_docs;

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
            return Ok(());
        }
    };

    let current_time_since_epoch = Utc::now().timestamp();

    let duration_in_seconds = duration.as_secs() as i64;

    let timestamp = current_time_since_epoch + duration_in_seconds;

    let database = Database::new("timer.db").unwrap();

    database.create_table_timer().unwrap();

    let user_id = ctx.author().id;
    let dm_channel = ctx.author().create_dm_channel(&ctx).await?;

    println!("user_id: {}", user_id);
    println!("dm_channel: {}", dm_channel.id);

    let msg = ctx
        .send(|m| {
            m.embed(|e| {
                e.title("Timer has been set")
                    .field("Description", &description, true)
                    .field("Time", &format!("<t:{}:R>", timestamp), true)
                    .color(0x00FF00)
            })
        })
        .await?;

    database
        .insert_timer(
            user_id.to_string().parse::<i64>().unwrap(),
            &description,
            timestamp,
            dm_channel.id.to_string().parse::<i64>().unwrap(),
            0,
        )
        .unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    msg.delete(ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let database = Database::new("timer.db").unwrap();

    database.create_table_timer().unwrap();

    match database.read_timer_by_uid(ctx.author().id.to_string().parse::<i64>().unwrap()) {
        Ok(data) => {
            if data.is_empty() {
                ctx.send(|m| m.content("You have no timers set.")).await?;
            }
            let mut counter = 0;
            let mut list_string = String::new();

            for (id, _, description, time, _, _) in data {
                counter += 1;
                // Append the formatted entry to the list string
                list_string.push_str(&format!(
                    "{}. id: {} | description: {} | time: <t:{}:R>\n",
                    counter, id, description, time
                ));
            }

            println!("{}", list_string);

            ctx.send(|m| {
                m.embed(|e| {
                    e.title("Timer list").description(format!(
                        "all of your timers are listed below: \n------------------\n{}",
                        &list_string
                    ))
                })
            })
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
    #[description = "what is the id of the timer you want to delete?"] data_id: i64,
) -> Result<(), Error> {
    let database = Database::new("timer.db").unwrap();

    database.create_table_timer().unwrap();

    match database.read_timer_by_uid(ctx.author().id.to_string().parse::<i64>().unwrap()) {
        Ok(data) => {
            if data.is_empty() {
                ctx.send(|m| m.content("You have no timers set.")).await?;
            }
            for (id, _, _, _, _, _) in data {
                if id == data_id {
                    database.delete_timer(id).unwrap();
                } else {
                    println!("id not found");
                }
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

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Help (coming soon)")
                .color(0x0000FF)
                .description(format!("{}", result))
        })
    })
    .await?;

    println!("{}", result);

    Ok(())
}
