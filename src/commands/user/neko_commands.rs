use crate::{Context, Error};

use crate::download_docs;

use poise::serenity_prelude as serenity;

use crate::commands::user::neko::{
    baka::baka, cuddle::cuddle, hug::hug, kiss::kiss, meow::meow, pat::pat, slap::slap,
};
use poise::CreateReply;
use serenity::all::colours;
use serenity::builder::CreateEmbed;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("baka", "cuddle", "hug", "kiss", "meow", "pat", "slap")
)]
///Get help for the neko commands
pub async fn neko(ctx: Context<'_>) -> Result<(), Error> {
    let result = download_docs::fetch_docs(&"commands/user/neko.md")
        .await
        .unwrap();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Help")
                .description(format!("{}", result))
                .color(colours::roles::DARK_RED),
        ),
    )
    .await?;

    println!("{}", result);
    Ok(())
}
