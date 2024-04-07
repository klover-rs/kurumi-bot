use crate::PrintError;
use crate::{Context, Error};
use std::io::Cursor;


use image::GenericImageView;

use regex::bytes;
use reqwest::Client;
use poise::serenity_prelude::{self as serenity, CreateAttachment};

use serenity::builder::CreateEmbed;


use poise::CreateReply;

#[derive(Debug, poise::ChoiceParameter)]
enum ImageFormat {
    Png,
    Jpeg,
    Webp
}



#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "the user you want to get the avatar from"] user: Option<serenity::User>,
    #[description = "the format you want to get the avatar in"] format: Option<ImageFormat>,
    #[description = "do you want to apply gray scale?"] grayscale: Option<bool>,
    #[description = "do you want to invert the images colours?"] invert: Option<bool>, 
    #[description = "do you want to apply a sepia tone to your image?"] sepia_tone: Option<bool>,
    #[description = "do you want to apply a blur effect to your image?"] blur: Option<u8>,
) -> Result<(), Error> {

    match user {
        Some(user) => {
            match user.avatar_url() {
                Some(url) => {
                    ctx.defer().await?;
                    if url.contains(".gif") {
                        ctx.send(
                            CreateReply::default().embed(CreateEmbed::default().description("this user has a gif avatar, if you applied filters, please note this is not possible with gifs.").image(url).color(serenity::colours::roles::DARK_RED)),
                        ).await?;
                        return Ok(());
                    }
                    let bytes = download_avatar(ctx, &url).await?;
                    match format {
                        Some(format) => {
                            use_filters(ctx, bytes, Some(format), grayscale, invert, sepia_tone, blur).await?;
                        }
                        None => {
                            use_filters(ctx, bytes, None, grayscale, invert, sepia_tone, blur).await?;
                        }
                    }
                }
                None => {
                    ctx.send(
                        CreateReply::default().embed(
                            CreateEmbed::default()
                                .title("Error")
                                .description("This user doesn't have an avatar")
                                .color(serenity::colours::roles::DARK_RED),
                        ),
                    )
                    .await?;
                    return Ok(());
                }
            }
        }
        None => {
            match ctx.author().avatar_url() {
                Some(url) => {
                    ctx.defer().await?;
                    let bytes = download_avatar(ctx, &url).await?;
                    match format {
                        Some(format) => {
                            use_filters(ctx, bytes, Some(format), grayscale, invert, sepia_tone, blur).await?;
                        }
                        None => {
                            use_filters(ctx, bytes, None, grayscale, invert, sepia_tone, blur).await?;
                        }
                    }
                    
                }
                None => {
                    ctx.send(CreateReply::default().content("No avatar found")).await?;
                }
            }
        }
    }

    Ok(())
}

async fn use_filters(ctx: Context<'_>, bytes: Vec<u8>, image_format: Option<ImageFormat>, grayscale: Option<bool>, invert: Option<bool>, sepia_tone: Option<bool>, blur: Option<u8>) -> Result<(), Error> {
    let mut edited_bytes = bytes.clone();
    match grayscale {
        Some(true) => {
            edited_bytes = apply_grayscale(ctx,  &image_format,bytes).await?;
                        
        }
        _ => {
                        
        }
    }

    match invert {
        Some(true) => {
            edited_bytes = apply_inverted(ctx, &image_format ,edited_bytes).await?;
        }
        _ => {
                        
        }
    }

    match sepia_tone {
        Some(true) => {
            edited_bytes = apply_sepia_tone(ctx, &image_format ,edited_bytes).await?;
        }
        _ => {}
    }

    match blur {
        Some(amount) => {
            edited_bytes = apply_blur(ctx, &image_format ,edited_bytes, amount as f32).await?;
        }
        _ => {}
    }
    
    let image_format = match image_format {
        Some(format) => match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Webp => "webp"
        }
        _ => "png"
    };
    
    ctx.send(CreateReply::default().content("Avatar").attachment(CreateAttachment::bytes(edited_bytes, format!("avatar.{}", image_format)))).await?;

    Ok(())
}

async fn apply_grayscale(ctx: Context<'_>, img_format: &Option<ImageFormat>, buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let img = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default().title("Error").description(format!("Failed to load image\nerror: {}", e)).color(0xFF0000)
            )).await?;
            return Err(Box::new(PrintError(format!("Error: {}", e))))
        }
    };

    let gray_img = img.grayscale();

    let mut output_buffer = Cursor::new(Vec::new());
    match img_format {
        Some(format) => {
            let format = match format {
                ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
                ImageFormat::Webp => image::ImageOutputFormat::WebP,
                ImageFormat::Png => image::ImageOutputFormat::Png,
            };

            match gray_img.write_to(&mut output_buffer, format) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
        None => {
            match gray_img.write_to(&mut output_buffer, image::ImageOutputFormat::Png) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
    }
}

async fn apply_inverted(ctx: Context<'_>,  img_format: &Option<ImageFormat>, buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let img = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default().title("Error").description(format!("Failed to load image\nerror: {}", e)).color(0xFF0000)
            )).await?;
            return Err(Box::new(PrintError(format!("Error: {}", e))))
        }
    };

    let (width, height) = img.dimensions();
    let mut inverted_img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let inverted_pixel = image::Rgb([
            255 - pixel[0],
            255 - pixel[1],
            255 - pixel[2],
        ]);
        inverted_img.put_pixel(x, y, inverted_pixel);
    } 

    let mut output_buffer = Cursor::new(Vec::new());

    match img_format {
        Some(format) => {
            let format = match format {
                ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
                ImageFormat::Webp => image::ImageOutputFormat::WebP,
                ImageFormat::Png => image::ImageOutputFormat::Png,
            };

            match inverted_img.write_to(&mut output_buffer, format) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
        None => {
            match inverted_img.write_to(&mut output_buffer, image::ImageOutputFormat::Png) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
    }
    
   

}

async fn apply_sepia_tone(
    ctx: Context<'_>,
    img_format: &Option<ImageFormat>,
    buffer: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let img = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default().title("Error").description(format!("Failed to load image\nerror: {}", e)).color(0xFF0000)
            )).await?;
            return Err(Box::new(PrintError(format!("Error: {}", e))));
        }
    };

    let mut sepia_tone_img = img.to_rgb8();
    
    for pixel in sepia_tone_img.pixels_mut() {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

        let tr = (0.393 * r as f32 + 0.769 * g as f32 + 0.189 * b as f32).min(255.0) as u8;
        let tg = (0.349 * r as f32 + 0.686 * g as f32 + 0.168 * b as f32).min(255.0) as u8;
        let tb = (0.272 * r as f32 + 0.534 * g as f32 + 0.131 * b as f32).min(255.0) as u8;

        *pixel = image::Rgb([tr, tg, tb]);

    }
    let mut output_buffer = Cursor::new(Vec::new());
    match img_format {
        Some(format) => {
            let format = match format {
                ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
                ImageFormat::Webp => image::ImageOutputFormat::WebP,
                ImageFormat::Png => image::ImageOutputFormat::Png,
            };

            match sepia_tone_img.write_to(&mut output_buffer, format) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
        None => {
            let mut output_buffer = Cursor::new(Vec::new());
            match sepia_tone_img.write_to(&mut output_buffer, image::ImageOutputFormat::Png) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to apply grayscale filter\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    return Err(Box::new(PrintError(format!("Error: {}", e))));
                }
            }
        }
    }

    
  

}

async fn apply_blur(ctx: Context<'_>, img_format: &Option<ImageFormat>, buffer: Vec<u8>, sigma: f32) -> Result<Vec<u8>, Error> {
    let img = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => {
            ctx.send(CreateReply::default().embed(
                CreateEmbed::default().title("Error").description(format!("Failed to load image\nerror: {}", e)).color(0xFF0000)
            )).await?;
            return Err(Box::new(PrintError(format!("Error: {}", e))))
        }
    };

    let blurred_img = img.blur(sigma);

    let mut output_buffer = Cursor::new(Vec::new());

    match img_format {
        Some(format) => {
            let format = match format {
                ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
                ImageFormat::Webp => image::ImageOutputFormat::WebP,
                ImageFormat::Png => image::ImageOutputFormat::Png,
            };
            
            match blurred_img.write_to(&mut output_buffer, format) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to save image\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    Err(Box::new(PrintError(format!("Error: {}", e))))
                }
            }
        }
        None => {
            let mut output_buffer = Cursor::new(Vec::new());
            match blurred_img.write_to(&mut output_buffer, image::ImageOutputFormat::Png) {
                Ok(_) => Ok(output_buffer.into_inner()),
                Err(e) => {
                    ctx.send(CreateReply::default().embed(
                        CreateEmbed::default().title("Error").description(format!("Failed to save image\nerror: {}", e)).color(0xFF0000)
                    )).await?;
                    Err(Box::new(PrintError(format!("Error: {}", e))))
                }
            }
        }
    }

}


async fn download_avatar(ctx: Context<'_>, url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
  
    if !response.status().is_success() {
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default().title("Error").description(format!("Failed to download avatar\nstatus: {}", response.status())).color(0xFF0000)
        )).await?;

        return Err(Box::new(PrintError(format!("Error: {}", response.status()))));
    }
  
    let bytes = response.bytes().await?;
  
    Ok(bytes.to_vec())
  }
  