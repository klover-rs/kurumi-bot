use chrono::Utc;
use poise::serenity_prelude as serenity;
use std::time::Duration;

use poise::CreateReply;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

pub async fn duration_timer(
    ctx: Context<'_>,
    unit: String,
    duration: String,
) -> Result<i64, Error> {
    let c_duration = match unit.as_str() {
        "s" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let seconds = match parts[0].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(seconds)
                }
                _ => {
                    let seconds = match parts[0].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(seconds)
                }
            }
        }
        "m" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let seconds = match parts[0].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(seconds * 60)
                }
                2 => {
                    let minutes = match parts[0].parse::<u64>() {
                        Ok(min) => min,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let seconds = match parts[1].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };

                    Duration::from_secs((minutes * 60) + seconds)
                }
                _ => {
                    let minutes = match parts[0].parse::<u64>() {
                        Ok(min) => min,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let seconds = match parts[1].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };

                    Duration::from_secs(minutes * 60 + seconds)
                }
            }
        }
        "h" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len() {
                1 => {
                    let hours = match parts[0].parse::<u64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(hours * 60 * 60)
                }
                2 => {
                    let hours = match parts[0].parse::<u64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let minutes = match parts[1].parse::<u64>() {
                        Ok(min) => min,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(hours * 60 * 60 + minutes * 60)
                }
                3 => {
                    let hours = match parts[2].parse::<u64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let minutes = match parts[0].parse::<u64>() {
                        Ok(min) => min,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let seconds = match parts[0].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds)
                }
                _ => {
                    let hours = match parts[0].parse::<u64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let minutes = match parts[1].parse::<u64>() {
                        Ok(min) => min,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    let seconds = match parts[2].parse::<u64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_error_msg(ctx, &e.to_string()).await.unwrap();
                            return Ok(0);
                        }
                    };
                    Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds)
                }
            }
        }
        _ => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description("Invalid unit. Only s, m, and h are supported")
                            .color(0xFF0000),
                    )
                    .ephemeral(true),
            )
            .await?;
            return Ok(0);
        }
    };

    let current_time_since_epoch = Utc::now().timestamp();

    let duration_in_seconds = c_duration.as_secs() as i64;

    let timestamp = current_time_since_epoch + duration_in_seconds;

    Ok(timestamp)
}

pub async fn set_timestamp(ctx: Context<'_>, unit: String, duration: String) -> Result<i64, Error> {
    let timestamp = duration_timer(ctx, unit, duration).await?;

    if timestamp == 0 {
        println!("Empty");
        return Ok(0);
    } else {
        println!("{}", timestamp);

        return Ok(timestamp);
    }
}

pub async fn send_error_msg(ctx: Context<'_>, error: &str) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Error")
                .description(format!("{}", error))
                .color(0xFF0000),
        ),
    )
    .await?;

    Ok(())
}
