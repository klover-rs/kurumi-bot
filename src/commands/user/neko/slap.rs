use std::vec;

use crate::{Context, Error};

use poise::CreateReply;

use poise::serenity_prelude as serenity;

use serenity::{
    all::colours,
    builder::{CreateEmbed, CreateEmbedAuthor},
};
///Slap someone
#[poise::command(slash_command, prefix_command)]
pub async fn slap(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let image_url = get_slap_image().await.expect("Failed to get slap image");
    let avatar_url = ctx.author().avatar_url().unwrap();

    let embed = CreateEmbed::new()
        .image(image_url)
        .author(
            CreateEmbedAuthor::new(format!("{} slapped {}!! ", ctx.author().name, &user.name))
                .icon_url(avatar_url),
        )
        .colour(colours::roles::BLUE);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn get_slap_image() -> Result<String, Box<dyn std::error::Error>> {
    let url: String = nekoslife::get(nekoslife::SfwCategory::Slap).await?;
    Ok(url)
}
