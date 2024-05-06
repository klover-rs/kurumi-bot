use std::collections::HashSet;

use crate::{Context, Error};

use image::error;
use poise::serenity_prelude::model::user;
use poise::serenity_prelude::RoleId;
use poise::CreateReply;

use poise::serenity_prelude as serenity;
use serenity::User;

use serenity::CreateEmbed;

use crate::db::user::xp::Database;

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("get_rank", "set_rank", "set_level_roles")
)]
pub async fn rank(ctx: Context<'_>) -> Result<(), Error> {

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

    //TODO: implement a check which checks, if a ordering number or roleID is already in use

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
            errors.push(format!("Duplicated index key {} at position {}", role_id, index + 1));
            continue;
        } else if !processed_role_ids.insert(role_id) {
            errors.push(format!("Duplicated role ID {} at position {}", role_id, index + 1));
            continue;
        }

        match guild_id.roles(ctx).await?.get(&RoleId::from(role_id as u64)) {
            Some(role) => {
                valid_roles.push((ordering_number, role.id));
            },
            None => {
                errors.push(format!("Role with ID {} not found", role_id));
            }
        };

    }

    if errors.is_empty() {
        println!("All roles validated successfully:");
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

    Ok(())
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
    
    println!("rank: {}", rank);
    println!("xp: {:?}", calculate_xp_from_rank(rank as i64));

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

fn calc_required_xp_for_nxt_rank(rank: i64) -> i64 {
    750 + 250 * rank
}