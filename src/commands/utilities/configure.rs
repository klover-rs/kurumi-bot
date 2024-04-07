use crate::{Context, Error};
use std::{fs, time::Instant};

use crate::db::configuration::Database;

use poise::{
    serenity_prelude::{self as serenity, model::channel, ChannelId},
    CreateReply,
};
use serenity::builder::CreateEmbed;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "ADMINISTRATOR",
    subcommands("upload", "set", "get", "clear")
)]
pub async fn configure(ctx: Context<'_>) -> Result<(), Error> {
    let result = fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("docs/commands/utilities/configure.md"),
    )
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
    #[description = "upload a configuration file (supported formats: .json, .toml)"]
    file: serenity::Attachment,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description("This command can only be used in guilds"),
                    )
                    .ephemeral(true),
            )
            .await?;
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
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!(
                            "an error occurred while trying to parse the json file:\n{}",
                            &e.to_string()
                        )),
                    ))
                    .await?;
                    return Ok(());
                }
            };

            let channel_id = match phrased.get("log_channel_id") {
                Some(v) => match v.as_str() {
                    Some(str_value) => match str_value.parse::<u64>() {
                        Ok(id) => ChannelId::from(id),
                        Err(_) => {
                            ctx.send(
                                CreateReply::default().embed(
                                    CreateEmbed::default()
                                        .title("Error")
                                        .description("log_channel_id is not a valid u64"),
                                ),
                            )
                            .await?;
                            return Ok(());
                        }
                    },
                    None => {
                        ctx.send(
                            CreateReply::default().embed(
                                CreateEmbed::default()
                                    .title("Error")
                                    .description("log_channel_id is not a string"),
                            ),
                        )
                        .await?;
                        return Ok(());
                    }
                },
                None => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(
                            "log_channel_id doesnt exist in your json file configuration.",
                        ),
                    ))
                    .await?;
                    return Ok(());
                }
            };

            //here we want to check if the channel id belongs to the guild we got the id from
            let guild_channel_id = match channel_id.to_channel(&ctx.http()).await {
                Ok(v) => match v.guild() {
                    Some(v) => v.guild_id,
                    None => {
                        ctx.send(
                            CreateReply::default().embed(
                                CreateEmbed::default()
                                    .title("Error")
                                    .description("log_channel_id is not in a guild"),
                            ),
                        )
                        .await?;
                        return Ok(());
                    }
                },
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!(
                            "an error occurred while trying to get the channel:\n{}",
                            &e.to_string()
                        )),
                    ))
                    .await?;
                    return Ok(());
                }
            };
            if guild_channel_id == guild_id {
                println!("channel is valid");
                insert_config(ctx, guild_id.try_into()?, channel_id.try_into()?).await?;
            }

            println!("{:?}", phrased);
        }
        "toml" => {
            println!("file format is toml");
            let toml = file.download().await?;

            let toml_str = String::from_utf8_lossy(&toml);

            let toml_data: toml::Value = match toml::from_str(toml_str.to_string().as_str()) {
                Ok(v) => v,
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!(
                            "an error occurred while trying to parse the toml file:\n{}",
                            &e.to_string()
                        )),
                    ))
                    .await?;
                    return Ok(());
                }
            };

            let channel_id = match toml_data.get("log_channel_id") {
                Some(v) => match v.as_str() {
                    Some(str_value) => match str_value.parse::<u64>() {
                        Ok(id) => ChannelId::from(id),
                        Err(_) => {
                            ctx.send(
                                CreateReply::default().embed(
                                    CreateEmbed::default()
                                        .title("Error")
                                        .description("log_channel_id is not a valid u64"),
                                ),
                            )
                            .await?;
                            return Ok(());
                        }
                    },
                    None => {
                        ctx.send(
                            CreateReply::default().embed(
                                CreateEmbed::default()
                                    .title("Error")
                                    .description("log_channel_id is not a string"),
                            ),
                        )
                        .await?;
                        return Ok(());
                    }
                },
                None => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(
                            "log_channel_id doesnt exist in your toml file configuration.",
                        ),
                    ))
                    .await?;
                    return Ok(());
                }
            };

            let guild_channel_id = match channel_id.to_channel(&ctx.http()).await {
                Ok(v) => match v.guild() {
                    Some(v) => v.guild_id,
                    None => {
                        ctx.send(
                            CreateReply::default().embed(
                                CreateEmbed::default()
                                    .title("Error")
                                    .description("log_channel_id is not in a guild"),
                            ),
                        )
                        .await?;
                        return Ok(());
                    }
                },
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!(
                            "an error occurred while trying to get the channel:\n{}",
                            &e.to_string()
                        )),
                    ))
                    .await?;
                    return Ok(());
                }
            };
            if guild_channel_id == guild_id {
                println!("channel is valid");
                insert_config(ctx, guild_id.try_into()?, channel_id.try_into()?).await?;
            }
        }
        _ => {
            println!("Unsupported file format");
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Unsupported file format.")
                        .description("Only .json and .toml files are supported."),
                ),
            )
            .await?;
        }
    }

    Ok(())
}

async fn insert_config(ctx: Context<'_>, guild_id: i64, log_channel: i64) -> Result<(), Error> {
    let db = Database::new().await?;
    db.create_table().await?;
    match db.insert(guild_id, log_channel).await {
        Ok(_) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Success")
                        .description("log channel has been set to this channel"),
                ),
            )
            .await?;
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let reply = {
                    let components = vec![serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("yes")
                            .style(serenity::ButtonStyle::Success)
                            .label("Yes"),
                        serenity::CreateButton::new("no")
                            .style(serenity::ButtonStyle::Danger)
                            .label("No"),
                    ])];

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
                            match db.update(guild_id, log_channel).await {
                                Ok(_) => {
                                    msg.edit(
                                        ctx,
                                        CreateReply::default()
                                            .content("updated configuration successfully")
                                            .components(vec![]),
                                    )
                                    .await?;
                                }
                                Err(e) => {
                                    msg.edit(
                                        ctx,
                                        CreateReply::default()
                                            .content(format!(
                                                "failed to update coonfiguration: {}",
                                                e.to_string()
                                            ))
                                            .components(vec![]),
                                    )
                                    .await?;
                                }
                            }
                        }
                        "no" => {
                            println!("no");
                            msg.edit(
                                ctx,
                                CreateReply::default()
                                    .content("operation cancelled")
                                    .components(vec![]),
                            )
                            .await?;
                        }
                        _ => {}
                    }
                    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                        .await?;

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
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description("this command can only be enforced in guilds."),
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let log_channel = log_channel.to_string().parse::<i64>().unwrap();

    insert_config(ctx, guild_id, log_channel).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description("this command can only be enforced in guilds."),
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let db = Database::new().await?;
    db.create_table().await?;
    let config = db.read_by_guild_id(guild_id).await?;
    if config.is_empty() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description("No configuration found"),
            ),
        )
        .await?;
        return Ok(());
    }

    ctx.send(
        CreateReply::default().embed(CreateEmbed::default().title("Configuration").field(
            "Log Channel",
            format!("{}", config[0].log_channel),
            true,
        )),
    )
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string().parse::<i64>().unwrap(),
        None => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description("this command can only be enforced in guilds."),
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let db = Database::new().await?;
    db.create_table().await?;
    match db.clear_by_guild_id(guild_id).await {
        Ok(_) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Success")
                        .description("configuration has been cleared"),
                ),
            )
            .await?;
        }
        Err(e) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description(format!("failed to clear configuration: {}", e.to_string())),
                ),
            )
            .await?;
        }
    }
    Ok(())
}
