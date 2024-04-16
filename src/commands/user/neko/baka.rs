
use std::vec;

use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::CreateReply;

use serenity::{
    all::colours,
    builder::{CreateEmbed, CreateEmbedAuthor},
};
///Call someone a baka
#[poise::command(slash_command, prefix_command)]
pub async fn baka(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let image_url = get_baka_image().await.expect("Failed to get baka image");
    let avatar_url = ctx.author().avatar_url().unwrap();

    let embed = CreateEmbed::new()
        .image(image_url)
        .author(
            CreateEmbedAuthor::new(format!(
                "{} called {} a baka ",
                ctx.author().name,
                &user.name
            ))
            .icon_url(avatar_url),
        )
        .colour(colours::roles::BLUE);

    ctx.send(
        CreateReply::default()
            .embed(embed)
            .content(format!("<@{}>", user.id)),
    )
    .await?;
    Ok(())
}

async fn get_baka_image() -> Result<String, Box<dyn std::error::Error>> {
    let url: String = nekoslife::get(nekoslife::SfwCategory::Baka).await?;
    Ok(url)
}
