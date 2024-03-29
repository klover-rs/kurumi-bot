use std::vec;

use crate::{Context, Error};
use poise::CreateReply;

use poise::serenity_prelude as serenity;

use serenity::{
    all::colours,
    builder::{CreateEmbed, CreateEmbedAuthor},
};
///Pat someone
#[poise::command(slash_command, prefix_command)]
pub async fn pat(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let image_url = get_pat_image().await.expect("Failed to get pat image");
    let avatar_url = ctx.author().avatar_url().unwrap();

    let embed = CreateEmbed::new()
        .image(image_url)
        .author(
            CreateEmbedAuthor::new(format!(
                "{} patted {}, dont pat too hard ",
                ctx.author().name,
                &user.name
            ))
            .icon_url(avatar_url),
        )
        .colour(colours::roles::BLUE);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn get_pat_image() -> Result<String, Box<dyn std::error::Error>> {
    let url: String = nekoslife::get(nekoslife::SfwCategory::Pat).await?;
    Ok(url)
}
