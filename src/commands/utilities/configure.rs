use crate::{Context, Error, PrintError};
use std::time::Instant;

use crate::db::configuration::Database;

use crate::download_docs;

use poise::{serenity_prelude::{self as serenity, model::channel, ChannelId}, CreateReply};
use serenity::builder::CreateEmbed;

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR", subcommands("upload", "set", "get", "clear"))]
pub async fn configure(
    ctx: Context<'_>,
) -> Result<(), Error> {
    
    let result = download_docs::fetch_docs(&"commands/utilities/configure.md")
        .await
        .unwrap();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Help")
                .description(format!("{}", result))
                .color(serenity::colours::roles::DARK_RED),
        ),
    )
    .await?;

    println!("{}", result);
    
    Ok(())
}


#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn upload(
    ctx: Context<'_>,
    #[description = "upload a configuration file (supported formats: .json, .toml)"] file: serenity::Attachment,
) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Error")
                    .description("This command can only be used in guilds")
                ).ephemeral(true)
            ).await?;
            return Ok(());
        }
    };

    let filename_parts = file.filename.split(".").collect::<Vec<&str>>();
    let extension = *filename_parts.last().unwrap();
    match extension {
        "json" => {
            println!("file format is json");

            let file_content = file.download().await?;

            let file_bytes = file_content.as_slice();

            let phrased: serde_json::Value = match serde_json::from_slice(file_bytes) {
                Ok(v) => v,
                Err(e) => {
                    ctx.send(
                        CreateReply::default().embed(CreateEmbed::default()
                            .title("Error")
                            .description(format!("an error occurred while trying to parse the json file:\n{}", &e.to_string()))
                        )
                    ).await?;
                    return Ok(());
                }
            };


            let channels_to_check = vec!["log_channel_id", "mod_log_channel_id"];
            let mut valid_channels = Vec::new();
            let mut errors: Vec<String> = Vec::new();

            for channel in channels_to_check {
                let channel_id = match pharse_channel_id_serde(&phrased, channel) {
                    Ok(v) => v,
                    Err(e) => {
                        let error_message = format!("Error processing channel '{}': {}", channel, e);
                        errors.push(error_message);
                        continue;
                    }
                };

                match channel_id {
                    Some(v) => {
                        let guild_channel_id = match v.to_channel(&ctx.http()).await {
                            Ok(v) => match v.clone().guild() {
                                Some(v) => v.guild_id,
                                None => {
                                    errors.push(format!("Error processing channel '{:?}': channel is not in a guild", v.id()));
                                    continue;
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Error processing channel '{}': {}", v, e));
                                continue;
                            }

                        };

                        if guild_channel_id == guild_id {
                            valid_channels.push(v.to_string());
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }

            for e in &errors {
                println!("error: {}",  e);
            }

            if errors.len() > 0 {
                ctx.send(
                    CreateReply::default().embed(CreateEmbed::default()
                        .title("Error")
                        .description(format!("An error occurred while processing the json file\n\n{}", errors.join("\n--------------\n")))
                    ).ephemeral(true)
                ).await?;
            
            }

            if valid_channels.len() > 0 {

                let log_channel = if valid_channels.len() > 0 {
                    Some(valid_channels[0].clone().parse::<i64>().unwrap())
                } else {
                    None
                };
                
                let mod_log_channel = if valid_channels.len() > 1 {
                    Some(valid_channels[1].clone().parse::<i64>().unwrap())
                } else {
                    None 
                };
                insert_config(ctx, guild_id.to_string().parse().unwrap(), log_channel, mod_log_channel).await?;

            }
            
        }
        "toml" => {
            println!("file format is toml");
            let toml = file.download().await?;

            let toml_str = String::from_utf8_lossy(&toml);

            let toml_data: toml::Value = match toml::from_str(toml_str.to_string().as_str()) {
                Ok(v) => v,
                Err(e) => {
                    ctx.send(
                        CreateReply::default().embed(CreateEmbed::default()
                            .title("Error")
                            .description(format!("an error occurred while trying to parse the toml file:\n{}", &e.to_string()))
                        )
                    ).await?;
                    return Ok(());
                }
            };

            let channels_to_check = vec!["log_channel_id", "mod_log_channel_id"];
            let mut valid_channels = Vec::new();
            let mut errors: Vec<String> = Vec::new();

            for channel in channels_to_check {
                let channel_id = match phrase_channel_id_toml(&toml_data, channel) {
                    Ok(v) => v,
                    Err(e) => {
                        let error_message = format!("Error processing channel '{}': {}", channel, e);
                        errors.push(error_message);
                        continue;
                    }
                };

                match channel_id {
                    Some(v) => {
                        let guild_channel_id = match v.to_channel(&ctx.http()).await {
                            Ok(v) => match v.clone().guild() {
                                Some(v) => v.guild_id,
                                None => {
                                    errors.push(format!("Error processing channel '{:?}': channel is not in a guild", v.id()));
                                    continue;
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Error processing channel '{}': {}", v, e));
                                continue;
                            }
                        };

                        if guild_channel_id == guild_id {
                            valid_channels.push(v.to_string());
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }

            for e in &errors {
                println!("error: {}",  e);
            }

            if errors.len() > 0 {
                ctx.send(
                    CreateReply::default().embed(CreateEmbed::default()
                        .title("Error")
                        .description(format!("An error occurred while processing the toml file\n\n{}", errors.join("\n--------------\n")))
                    ).ephemeral(true)
                ).await?;
            
            }

            if valid_channels.len() > 0 {

                let log_channel = if valid_channels.len() > 0 {
                    Some(valid_channels[0].clone().parse::<i64>().unwrap())
                } else {
                    None
                };
                
                let mod_log_channel = if valid_channels.len() > 1 {
                    Some(valid_channels[1].clone().parse::<i64>().unwrap())
                } else {
                    None 
                };
                insert_config(ctx, guild_id.to_string().parse().unwrap(), log_channel, mod_log_channel).await?;

            }
        }
        _ => {
            println!("Unsupported file format");
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Unsupported file format.")
                    .description("Only .json and .toml files are supported.")
            )).await?;
        }
    }

    Ok(())
}

fn pharse_channel_id_serde(value: &serde_json::Value, key: &str) -> Result<Option<ChannelId>, Error> {

    match value.get(key) {
        Some(v) => {
            match v.as_str() {
                Some(str_value) => {
                    match str_value.parse::<u64>() {
                        Ok(id) => Ok(Some(ChannelId::from(id))),
                        Err(e) => {
                            
                            return Err(Box::new(PrintError(format!("\nJson Error: {}", e))));
                        }
                    }
                }
                None => {
                    return Err(Box::new(PrintError(format!("\nJson Error: {}", "not a string"))));
                }
            }
        }
        None => Ok(None),
    }

}


fn phrase_channel_id_toml(value: &toml::Value, key: &str) -> Result<Option<ChannelId>, Error> {

    match value.get(key) {
        Some(v) => {
            match v.as_str() {
                Some(str_value) => {
                    match str_value.parse::<u64>() {
                        Ok(id) => Ok(Some(ChannelId::from(id))),
                        Err(e) => {
                            return Err(Box::new(PrintError(format!("\nJson Error: {}", e))));
                        }
                    }
                }
                None => {
                    return Err(Box::new(PrintError(format!("\nJson Error: {}", "not a string"))));
                }
            }
        }
        None => Ok(None),
    }

}


async fn insert_config(ctx: Context<'_>, guild_id: i64, log_channel: Option<i64>, mod_log_channel: Option<i64>) -> Result<(), Error> {

    let db = Database::new().await?;
    db.create_table().await?;
    match db.insert(guild_id, log_channel, mod_log_channel).await {
        Ok(_) => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Success")
                    .description("Comfiguration has been updated.")
                )
            ).await?;
        },
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                let reply = {
                    let components = vec![
                        serenity::CreateActionRow::Buttons(vec![
                            serenity::CreateButton::new("yes")
                                .style(serenity::ButtonStyle::Success)
                                .label("Yes"),
                            serenity::CreateButton::new("no")
                                .style(serenity::ButtonStyle::Danger)
                                .label("No"),
                        ])
                    ];

                    CreateReply::default()
                        .content("You have already an existing configuration, do you want to replace it with your new provided parameters?")
                        .components(components)
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
                            println!("yes");
                            match db.update(guild_id, log_channel, mod_log_channel).await {
                                Ok(_) => {
                                    msg.edit(ctx, CreateReply::default().content(
                                        "updated configuration successfully"
                                    ).components(vec![])).await?;
                                },
                                Err(e) => {
                                    msg.edit(ctx, CreateReply::default().content(
                                        format!("failed to update coonfiguration: {}", e.to_string())
                                    ).components(vec![])).await?;
                                },
                            }

                            
                        }
                        "no" => {
                            println!("no");
                            msg.edit(ctx, CreateReply::default().content("operation cancelled").components(vec![])).await?;
                        }
                        _ => {}
                    }
                    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;

                    break;
                }
            }
            println!("{:?}", e);
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn set(
    ctx: Context<'_>,
    #[description = "log channel"] log_channel: serenity::ChannelId,
) -> Result<(), Error> {

    println!("{:?}", log_channel);

    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Error")
                    .description("this command can only be enforced in guilds.")
                )
            ).await?;
            return Ok(());
        }
    };

    let log_channel = log_channel.to_string().parse::<i64>().unwrap();
    
    insert_config(ctx, guild_id, Some(log_channel), None).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Error")
                    .description("this command can only be enforced in guilds.")
                )
            ).await?;
            return Ok(());
        }
    };

    let db = Database::new().await?;
    db.create_table().await?;
    let config = db.read_by_guild_id(guild_id).await?;
    if config.is_empty() {
        ctx.send(
            CreateReply::default().embed(CreateEmbed::default()
                .title("Error")
                .description("No configuration found")
            )
        ).await?;
        return Ok(());
    }
    
    ctx.send(
        CreateReply::default().embed(CreateEmbed::default()
            .title("Configuration")
            .field("Log Channel", format!("{}", config[0].log_channel), true)
        )
    ).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Error")
                    .description("this command can only be enforced in guilds.")
                )
            ).await?;
            return Ok(());
        }
    };

    let db = Database::new().await?;
    db.create_table().await?;
    match db.clear_by_guild_id(guild_id).await {
        Ok(_) => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Success")
                    .description("configuration has been cleared")
                )
            ).await?;
        },
        Err(e) => {
            ctx.send(
                CreateReply::default().embed(CreateEmbed::default()
                    .title("Error")
                    .description(format!("failed to clear configuration: {}", e.to_string()))
                )
            ).await?;
        }
    }
    Ok(())
}