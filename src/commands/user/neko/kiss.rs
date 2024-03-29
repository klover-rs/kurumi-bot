use std::vec;

use crate::{Context, Error};

use poise::{serenity_prelude as serenity, serenity_prelude::CreateMessage};

use serenity::{
    all::colours,
    builder::{CreateEmbed, CreateEmbedAuthor},
};
///Kiss someone
#[poise::command(slash_command, prefix_command)]
pub async fn kiss(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let image_url = get_kiss_image().await.expect("Failed to get kiss image");
    let avatar_url = ctx.author().avatar_url().unwrap();

    let embed = CreateEmbed::new()
        .image(image_url)
        .author(
            CreateEmbedAuthor::new(format!("{} kissed {}!! ", ctx.author().name, &user.name))
                .icon_url(avatar_url),
        )
        .colour(colours::roles::BLUE);
    let builder = CreateMessage::new().embed(embed);
    ctx.channel_id().send_message(&ctx.http(), builder).await?;
    Ok(())
}

async fn get_kiss_image() -> Result<String, Box<dyn std::error::Error>> {
    let url: String = nekoslife::get(nekoslife::SfwCategory::Kiss).await?;
    Ok(url)
}
