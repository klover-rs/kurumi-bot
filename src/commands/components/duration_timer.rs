use poise::serenity_prelude as serenity;
use chrono::{Utc, Duration};


use std::num::ParseIntError;

use serenity::builder::CreateEmbed;
use poise::CreateReply;

use crate::{Context, Error};

pub async fn duration_timer(
    ctx: Context<'_>,
    unit: String,
    duration: String,      
) -> Result<i64, Error> {
    

    match unit.as_str() {
        "s" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len()
            {
                1 => {
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    match Duration::try_seconds(seconds) {
                        Some(duration) => {
                            duration.num_seconds();
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                    
                }
                _ => {
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    match Duration::try_seconds(seconds) {
                        Some(duration) => {
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                } 
            }
            
        }
        "m" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len()
            {
                1 => {
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };

                    match Duration::try_seconds(seconds * 60) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                }
                2 => {
                    let minutes = match parts[0].parse::<i64>() {
                        Ok(minutes) => minutes,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };

                    match Duration::try_seconds(minutes * 60 + seconds) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                    
                }
                _ => {
                    let minutes = parts[0].parse::<i64>().unwrap();
                    let seconds = parts[1].parse::<i64>().unwrap();

                    match Duration::try_seconds(minutes * 60 + seconds) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                }
            }
        }
        "h" => {
            let parts = duration.split(':').collect::<Vec<&str>>();

            match parts.len()
            {
                1 => {
                    let hours = match parts[0].parse::<i64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };

                    match Duration::try_seconds(hours * 60 * 60) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                }
                2 => {
                    let hours = match parts[0].parse::<i64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let minutes = match parts[0].parse::<i64>() {
                        Ok(minutes) => minutes,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    
                    match Duration::try_seconds(hours * 60 * 60 + minutes * 60) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }
                }
                3 => {
                    let hours = match parts[0].parse::<i64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let minutes = match parts[0].parse::<i64>() {
                        Ok(minutes) => minutes,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };

                    match Duration::try_seconds(hours * 60 * 60 + minutes * 60 + seconds) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }

                }
                _ => {
                    let hours = match parts[0].parse::<i64>() {
                        Ok(hours) => hours,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let minutes = match parts[0].parse::<i64>() {
                        Ok(minutes) => minutes,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };
                    let seconds = match parts[0].parse::<i64>() {
                        Ok(seconds) => seconds,
                        Err(e) => {
                            send_parse_error(ctx, e).await?;
                            return Ok(0)
                        }
                    };

                    match Duration::try_seconds(hours * 60 * 60 + minutes * 60 + seconds) {
                        Some(duration) => {
                            
                            return Ok(duration.num_seconds())
                        }
                        None => {
                            return Ok(0)
                        }
                    }

                }
            }
        }
        _ => {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                .title("Error")
                .description("Invalid unit. Only s, m, and h are supported")
                .color(0xFF0000)
            ).ephemeral(true)).await?;
            return Ok(0);
        }
    };

}

pub async fn set_timestamp(ctx: Context<'_>, unit: String, duration: String) -> Result<(), Error> {

    let c_duration = duration_timer(ctx, unit, duration).await?;

    if c_duration == 0 {
        println!("Empty");
    } else {
        println!("{}", c_duration);
        let current_time = Utc::now();

        let timestamp = current_time.timestamp() + c_duration;

        println!("{}", timestamp);
    }
    
    Ok(())
}

pub async fn send_parse_error(ctx: Context<'_>, error: ParseIntError) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(format!("parse error\n{}", error))).await?;

    Ok(())
}