use std::collections::HashSet;
use std::str::FromStr;

use crate::{Context, Error};

use poise::serenity_prelude::CreateAttachment;
use poise::serenity_prelude::GuildId;
use poise::serenity_prelude::Member;
use poise::serenity_prelude::RoleId;
use poise::serenity_prelude::UserId;

use crate::secrets::get_secret;

use poise::CreateReply;

use poise::serenity_prelude as serenity;
use serenity::User;

use serenity::CreateEmbed;

use crate::db::user::xp::Database;

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("get_rank", "set_rank", "set_level_roles", "xp_leaderboard", "clear_level_roles", "upload_level_roles")
)]
pub async fn rank(ctx: Context<'_>) -> Result<(), Error> {

    Ok(())
}

#[poise::command(
    prefix_command, 
    slash_command,
    
)]
pub async fn upload_level_roles(
    ctx: Context<'_>,
    #[description = "upload a file containing your level roles (supported formats: .json, .toml)"] file: serenity::Attachment
) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
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

    let filename_parts = file.filename.split(".").collect::<Vec<&str>>();
    let extension = *filename_parts.last().unwrap();

    let mut valid_roles: Vec<(i32, RoleId)> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut processed_ordering_ids: HashSet<i32> = HashSet::new();
    let mut processed_role_ids: HashSet<i64> = HashSet::new();
    let bot_uid = UserId::from_str(get_secret("APP_ID").as_str()).unwrap();
    let bot_roles = match get_member(&ctx, guild_id, bot_uid).await {
        Some(member) => {
            
            match member.roles(ctx) {
                Some(roles) => roles,
                None => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description("The bot needs to have at least one role.")
                    )).await?;
                    return Ok(())
                }
            }
        },
        None => return Ok(()),
    };

    let highest_role = bot_roles.iter().max_by_key(|role| role.position).unwrap();


    let file_content = file.download().await?;

    let file_bytes = file_content.as_slice();
    let file_string = match std::str::from_utf8(file_bytes) {
        Ok(v) => v.chars().filter(|&c| !c.is_whitespace()).collect::<String>(),
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

    println!("file string: {:?}", &file_string);

    match extension {
        "json" => {
            let phrased_json: serde_json::Value = match serde_json::from_str(&file_string) {
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


            if let serde_json::Value::Object(map) = phrased_json {

                let mut index = 0;

                for (key, value) in map.iter() {
                    index += 1;
                    let key = match key.parse::<i32>() {
                        Ok(ordering_number) => ordering_number,
                        Err(e) => {
                            errors.push(format!("Error on line {}: {}", index, e.to_string()));
                            println!("{:?}", key);
                            continue;
                        }
                    };
                    let role_id_number = match value.to_string().trim_matches('"').parse::<i64>() {
                        Ok(role_id) => role_id,
                        Err(e) => {
                            errors.push(format!("Error on line {}: {}", index, e.to_string()));
                            continue;
                        }
                    };

                    if !processed_ordering_ids.insert(key) {
                        errors.push(format!("Error on line {}: Ordering number {} is not unique", index, key));
                        continue;
                    } else if !processed_role_ids.insert(role_id_number) {
                        errors.push(format!("Error on line {}: role id {} is not unique", index, role_id_number));
                        continue;
                    }

                    match guild_id.roles(ctx).await?.get(&RoleId::from(role_id_number as u64)) {
                        Some(role) => {
                            if highest_role.position < role.position {
                                errors.push(format!("Role with ID {} is higher than the highest role of the bot", role_id_number));
                            } else {
                                valid_roles.push((index, role.id));
                            }
                        }
                        None => {
                            errors.push(format!("Role with ID {} not found", role_id_number));
                        }
                    }

                }
            }
        }
        "toml" => {
            let toml_data: toml::Value = match toml::from_str(&file_string) {
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

            if let toml::Value::Table(map) = toml_data {
                let mut index = 0;

                for (key, value) in map.iter() {
                    index += 1;
                    let key = match key.parse::<i32>() {
                        Ok(ordering_number) => ordering_number,
                        Err(e) => {
                            errors.push(format!("Error on line {}: {}", index, e.to_string()));
                            continue;
                        }
                    };

                    let role_id_number = match value.to_string().trim_matches('"').parse::<i64>() {
                        Ok(role_id) => role_id,
                        Err(e) => {
                            errors.push(format!("Error on line {}: {}", index, e.to_string()));
                            continue;
                        }
                    };

                    if !processed_ordering_ids.insert(key) {
                        errors.push(format!("Error on line {}: Ordering number {} is not unique", index, key));
                        continue;
                    } else if !processed_role_ids.insert(role_id_number) {
                        errors.push(format!("Error on line {}: role id {} is not unique", index, role_id_number));
                        continue;
                    }

                    match guild_id.roles(ctx).await?.get(&RoleId::from(role_id_number as u64)) {
                        Some(role) => {
                            if highest_role.position < role.position {
                                errors.push(format!("Role with ID {} is higher than the highest role of the bot", role_id_number));
                            } else {
                                valid_roles.push((index, role.id));
                            }
                        }
                        None => {
                            errors.push(format!("Role with ID {} not found", role_id_number));
                        }
                    }
                    
                }
            }
        }
        _ => {
            println!("Unsupported file format");
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Unsupported file format.")
                    .description("Only .json and .toml files are supported.")
            )).await?;
            return Ok(());
        }

    }

    if errors.is_empty() {
        println!("All roles validated successfully:");
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default()
                .title("Result")
                .description(format!("All roles validated successfully: \n{:?}", &valid_roles))
                .color(0x77ff01)
        )).await?;
    } else {
        println!("Errors encountered during validation:");
        if valid_roles.is_empty() {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!("No valid roles found, encountered error: \n{}", &errors.join("\n---------\n")))
                    .color(0xff0000)
            )).await?;
            return Ok(());
        } else {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Result")
                    .description(format!("The following roles have been validated:\n{:?}\n**Errors:**\n{}", &valid_roles, &errors.join("\n---------\n")))
                    .color(0xffff00)
            )).await?;
        }
    }

    let mut valid_roles_str = String::new();
    for (ordering_number, role_id) in valid_roles {
        let role_id_str = role_id.to_string();
        valid_roles_str.push_str(&format!("{}={},", ordering_number, role_id_str));
    }
    valid_roles_str.pop();

    insert_level_roles(ctx, guild_id, &valid_roles_str).await?;

    Ok(())
}


#[poise::command(
    prefix_command, 
    slash_command,
)]
pub async fn set_level_roles(
    ctx: Context<'_>,
    #[description = "assign level roles to specific ranks e.g. (5=role_id,10=role_id) without brackets"] roles: String,
) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
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

    let roles: String = roles.chars().filter(|&c| !c.is_whitespace()).collect();

    let mut valid_roles: Vec<(i32, RoleId)> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut processed_ordering_ids: HashSet<i32> = HashSet::new();
    let mut processed_role_ids: HashSet<i64> = HashSet::new();
    let bot_uid = UserId::from_str(get_secret("APP_ID").as_str()).unwrap();
    let bot_roles = match get_member(&ctx, guild_id, bot_uid).await {
        Some(member) => {
            
            match member.roles(ctx) {
                Some(roles) => roles,
                None => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description("The bot needs to have at least one role.")
                    )).await?;
                    return Ok(())
                }
            }
        },
        None => return Ok(()),
    };

    let highest_role = bot_roles.iter().max_by_key(|role| role.position).unwrap();

    for (index, role) in roles.split(',').enumerate() {
        let parts: Vec<&str> = role.split('=').collect();
        if parts.len() != 2 {
            errors.push(format!("Invalid syntax at position {}: Expected 'ordering=role_id' format", index + 1));
            continue;
        }
        
        let ordering_number = parts[0].parse::<i32>();
        let role_id = parts[1].parse::<i64>();

        if ordering_number.is_err() || role_id.is_err() {
            errors.push(format!("Invalid values at position {}: Both ordering and role ID must be numeric", index + 1));
            continue;
        }

        let role_id = role_id.unwrap();
        let ordering_number = ordering_number.unwrap();

        if !processed_ordering_ids.insert(ordering_number) {
            errors.push(format!("Duplicated index key {} at position {}", ordering_number, index + 1));
            continue;
        } else if !processed_role_ids.insert(role_id) {
            errors.push(format!("Duplicated role ID {} at position {}", role_id, index + 1));
            continue;
        }

        match guild_id.roles(ctx).await?.get(&RoleId::from(role_id as u64)) {
            Some(role) => {
                if highest_role.position < role.position {
                    errors.push(format!("Role with ID {} is higher than the highest role of the bot", role_id));
                } else {
                    valid_roles.push((ordering_number, role.id));
                }
            },
            None => {
                errors.push(format!("Role with ID {} not found", role_id));
            }
        };

    }

    if errors.is_empty() {
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default()
                .title("Result")
                .description(format!("All roles validated successfully: \n{:?}", &valid_roles))
        )).await?;
    } else {
        println!("Errors encountered during validation:");
        if valid_roles.is_empty() {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!("No valid roles found, encountered error: \n{}", &errors.join("\n---------\n")))
            )).await?;
            return Ok(());
        } else {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Result")
                    .description(format!("The following roles have been validated:\n{:?}\n**Errors:**\n{}", &valid_roles, &errors.join("\n---------\n")))
            )).await?;
        }
    }

    let mut valid_roles_str = String::new();
    for (ordering_number, role_id) in valid_roles {
        let role_id_str = role_id.to_string();
        valid_roles_str.push_str(&format!("{}={},", ordering_number, role_id_str));
    }
    valid_roles_str.pop();

    insert_level_roles(ctx, guild_id, &valid_roles_str).await?;

    Ok(())
}

async fn insert_level_roles(ctx: Context<'_>, guild_id: GuildId, lvl_roles: &str) -> Result<(), Error> {
    let db = Database::new().await?;

    db.create_table_level_roles().await?;
    
    match db.insert_level_roles(guild_id.try_into()?, lvl_roles).await {
        Ok(_) => {
            ctx.send(CreateReply::default().embed(CreateEmbed::default()
                .title("Success")
                .description("Level roles have been set successfully")
            )).await?;
            Ok(())
        },
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                let components = vec![
                    serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("yes")
                            .style(serenity::ButtonStyle::Success)
                            .label("Yes"),
                        serenity::CreateButton::new("no")
                            .style(serenity::ButtonStyle::Danger)
                            .label("No"),
                        serenity::CreateButton::new("download_backup")
                            .style(serenity::ButtonStyle::Primary)
                            .label("Download Backup"),
                    ])
                ];
                let embed = CreateEmbed::default()
                    .title("Warning")
                    .description("You already have an existing configuration, do you want to replace it with your new provided parameters?")
                    .color(0xff0000);

                let reply = CreateReply::default()
                    .embed(embed)
                    .components(components);
                
                let msg = ctx.send(reply).await?;

                if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.clone())
                    .author_id(ctx.author().id)
                    .channel_id(ctx.channel_id())
                    .timeout(std::time::Duration::from_secs(120))
                    .await
                {
                    match mci.data.custom_id.as_str() {
                        "yes" => {
                            match db.update_level_roles(guild_id.try_into()?, lvl_roles).await {
                                Ok(_) => {
                                    msg.edit(ctx, CreateReply::default().content("Updated level roles successfully").components(vec![])).await?;
                                },
                                Err(e) => {
                                    msg.edit(ctx, CreateReply::default().content(format!("Failed to update level roles: {}", e.to_string())).components(vec![])).await?;
                                },
                            }
                        },
                        "no" => {
                            msg.edit(ctx, CreateReply::default().content("Operation cancelled").components(vec![])).await?;
                        },
                        "download_backup" => {
                            let backup = db.read_level_roles(guild_id.try_into()?).await?;
                            msg.edit(ctx, CreateReply::default().attachment(CreateAttachment::bytes(backup.as_bytes(), "level_roles.txt")).components(vec![])).await?;
                            println!("backup: {:?}", backup);
                        }
                        _ => {}
                    }
                    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                }
            }
            Ok(()) 
        }
    }
}
async fn get_member(ctx: &Context<'_>, guild_id: GuildId, user_id: UserId) -> Option<Member> {
    if let Some(member) = guild_id.member(&ctx, user_id).await.ok() {
        Some(member)
    } else {
        guild_id.member(&ctx, user_id).await.ok()
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn clear_level_roles(ctx: Context<'_>) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
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
    match db.clear_level_roles(guild_id.try_into()?).await {
        Ok(_) => {
            ctx.send(CreateReply::default().embed(CreateEmbed::default()
                .title("Success")
                .description("Cleared level roles successfully")
            )).await?;
            return Ok(())
        },
        Err(e) => {
            ctx.send(CreateReply::default().embed(CreateEmbed::default()
                .title("Error")
                .description(format!("Failed to clear level roles: {}", e.to_string()))
            )).await?;
            return Ok(())
        }
    };

}

#[poise::command(prefix_command, slash_command)]
pub async fn set_rank(ctx: Context<'_>, user: User, rank: u16) -> Result<(), Error> {

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
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

    if rank == 0 {
        ctx.send(
            CreateReply::default().embed(CreateEmbed::default()
                .title("Error")
                .description("Rank cannot be 0")
            ).ephemeral(true)
        ).await?;
        return Ok(());
    }

    let (xp, xp_in_this_rank) = calculate_xp_from_rank(rank as i64);

    let db = Database::new().await?;

    db.create_table().await?;

    db.update(user.id.get() as i64, guild_id.get() as i64, xp, xp_in_this_rank, rank as i64).await?;

    ctx.send(
        CreateReply::default().embed(CreateEmbed::default()
            .title(format!("{}'s rank has been set to {}", user.name, rank))
        )
    ).await?;

    Ok(())
}

fn calculate_xp_from_rank(rank: i64) -> (i64, i64) {
    (1000 + ((rank - 1) * 250), 0)
}

#[poise::command(prefix_command, slash_command)]
pub async fn get_rank(
    ctx: Context<'_>,
    user: Option<User>
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

    let db = Database::new().await?;

    db.create_table().await?;

    if let Some(user) = user {
        let xp = db.read(user.id.try_into()?, guild_id.try_into()?).await?;
        
        let embed = CreateEmbed::default()
            .title(format!("{}'s rank", user.name))
            .description(format!("**Rank**: {}\n**XP**: {}\n{} **XP left** to next rank", xp[0].rank, xp[0].xp, xp[0].xp_in_this_rank))
            .color(serenity::colours::roles::DARK_RED);

        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        let xp = db.read(ctx.author().id.try_into()?, guild_id.try_into()?).await?;

        let embed = CreateEmbed::default()
            .title("Your rank")
            .description(format!("**Rank**: {}\n**Total XP**: {}\n**XP to next rank**: {}/{} xp", xp[0].rank, xp[0].xp, xp[0].xp_in_this_rank, calc_required_xp_for_nxt_rank(xp[0].rank)))
            .color(serenity::colours::roles::DARK_RED);

        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn xp_leaderboard(ctx: Context<'_>) -> Result<(), Error> {

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

    let db = Database::new().await?;
    db.create_table().await?;

    let top_10_leaderboard = db.top_10_xp(guild_id.try_into()?).await?;

    let mut leaderboard_text = String::new();
    for (index, xp_record) in top_10_leaderboard.iter().enumerate() {
        leaderboard_text.push_str(&format!(
            "({}) **User**: <@{}>, **rank**: {}, **XP**: {}\n",
            index + 1,
            xp_record.uid,
            xp_record.rank, 
            xp_record.xp
        ));
    }

    let embed = CreateEmbed::default()
        .title("Top 10 Leaderboard")
        .description(leaderboard_text)
        .color(serenity::colours::roles::DARK_RED);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

fn calc_required_xp_for_nxt_rank(rank: i64) -> i64 {
    750 + 250 * rank
}