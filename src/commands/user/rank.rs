use crate::{Context, Error};

use poise::serenity_prelude::model::user;
use poise::CreateReply;

use poise::serenity_prelude as serenity;
use serenity::User;

use serenity::CreateEmbed;

use crate::db::user::xp::Database;

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