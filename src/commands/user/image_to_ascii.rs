use crate::{Context, Error};
use std::time::Instant;

use poise::serenity_prelude::{CreateAttachment, CreateEmbed};
use poise::CreateReply;

use fastrand::Rng;

use poise::serenity_prelude as serenity;
use image_to_ascii::convert_to_ascii;

/// Shows the bot's latency
#[poise::command(prefix_command, slash_command)]
pub async fn image_to_ascii(
    ctx: Context<'_>,
    #[description = "the image you want to convert to ascii"] image: serenity::Attachment,
    #[description = "the quality of the image (default = 80, max = 255)"] quality: u8,
) -> Result<(), Error> {

    let msg = ctx.send(
        CreateReply::default().content("processing.. this may take a while").embed(
            CreateEmbed::default()
            .title("Beta notice!")
            .description("This command is still in beta. Results may not be as accurate as expected to be.")
        ).ephemeral(true)
    ).await?;
    
    println!("{}", image.filename);

    let image_dl = image.download().await.unwrap();

    let filename_parts = image.filename.split(".").collect::<Vec<&str>>();
    let extension = *filename_parts.last().unwrap();

    match extension {
        "png" | "jpg" | "jpeg" => {
            let start = Instant::now();
            let ascii = convert_to_ascii(image_dl, 200).unwrap();
            let end = Instant::now();
            let duration = end - start;
            ctx.send(CreateReply::default()
                .attachment(CreateAttachment::bytes(ascii, format!("{}.png", generate_random_string())))
                .content(format!("Your image has been processed <@{}>! (processed in {} ms)", ctx.author().id, duration.as_millis()))
            ).await?;
        }

        _ => {
            println!("Unsupported image format");
        }
    }

    Ok(())
}

fn generate_random_string() -> String {
    let mut rng = Rng::new();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut result = String::with_capacity(16);

    for _ in 0..8 {
        let idx = rng.usize(..charset.len());
        result.push(charset[idx] as char);
    }

    result
}