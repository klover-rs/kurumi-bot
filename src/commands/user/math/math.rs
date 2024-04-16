use crate::{Context, Error};

use crate::commands::user::math::square_roots::square_roots;

#[poise::command(prefix_command, slash_command, subcommands("square_roots"))]
pub async fn math(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("please call one of the subcommands").await?;
    Ok(())
}