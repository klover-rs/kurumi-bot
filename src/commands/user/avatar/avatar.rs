use crate::commands::user::avatar::{
    gpu_init::DEVICE_QUEUE, grayscale::apply_grayscale, invert::apply_invert, sepia::apply_sepia,
};
use crate::PrintError;
use crate::{Context, Error};
use std::io::Cursor;

use poise::serenity_prelude::{self as serenity, CreateAttachment};
use reqwest::Client;

use serenity::builder::CreateEmbed;

use poise::CreateReply;

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
enum ImageFormat {
    Png,
    Jpeg,
    Webp,
}

#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "the user you want to get the avatar from"] user: Option<serenity::User>,
    #[description = "the format you want to get the avatar in"] format: Option<ImageFormat>,
    #[description = "do you want to apply gray scale?"] grayscale: Option<bool>,
    #[description = "do you want to invert the images colours?"] invert: Option<bool>,
    #[description = "do you want to apply a sepia tone to your image?"] sepia_tone: Option<bool>,
) -> Result<(), Error> {
    match user {
        Some(user) => match user.avatar_url() {
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
                        use_filters(ctx, bytes, Some(format), grayscale, invert, sepia_tone)
                            .await?;
                    }
                    None => {
                        use_filters(ctx, bytes, None, grayscale, invert, sepia_tone).await?;
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
        },
        None => match ctx.author().avatar_url() {
            Some(url) => {
                ctx.defer().await?;
                let bytes = download_avatar(ctx, &url).await?;
                match format {
                    Some(format) => {
                        use_filters(ctx, bytes, Some(format), grayscale, invert, sepia_tone)
                            .await?;
                    }
                    None => {
                        use_filters(ctx, bytes, None, grayscale, invert, sepia_tone).await?;
                    }
                }
            }
            None => {
                ctx.send(CreateReply::default().content("No avatar found"))
                    .await?;
            }
        },
    }

    Ok(())
}

async fn use_filters(
    ctx: Context<'_>,
    bytes: Vec<u8>,
    image_format: Option<ImageFormat>,
    grayscale: Option<bool>,
    invert: Option<bool>,
    sepia_tone: Option<bool>,
) -> Result<(), Error> {
    let start = std::time::Instant::now();

    let device = &DEVICE_QUEUE.0;
    let queue = &DEVICE_QUEUE.1;

    let mut img = match image::load_from_memory(&bytes) {
        Ok(img) => img.to_rgba8(),
        Err(err) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description(err.to_string())
                        .color(serenity::colours::roles::DARK_RED),
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let (width, height) = img.dimensions();

    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let input_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("input texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("output texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
    });

    let mut used_encoder = String::from("default");

    match grayscale {
        Some(true) => {
            apply_grayscale(
                &mut img,
                width,
                height,
                &input_texture,
                &output_texture,
                &texture_size,
                &device,
                &queue,
            )?;
        }
        _ => {}
    }

    match invert {
        Some(true) => {
            apply_invert(
                &mut img,
                width,
                height,
                &input_texture,
                &output_texture,
                &texture_size,
                &device,
                &queue,
            )?;
            used_encoder.push_str("gpu acceleration (unoptimized)");
        }
        _ => {}
    }

    match sepia_tone {
        Some(true) => {
            apply_sepia(
                &mut img,
                width,
                height,
                &input_texture,
                &output_texture,
                &texture_size,
                &device,
                &queue,
            )?;
            used_encoder.push_str("gpu acceleration (unoptimized)");
        }
        _ => {}
    }

    let image_format_str = match image_format {
        Some(format) => match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Webp => "webp",
        },
        _ => "png",
    };
    let image_format = match image_format {
        Some(format) => match format {
            ImageFormat::Png => image::ImageOutputFormat::Png,
            ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
            ImageFormat::Webp => image::ImageOutputFormat::WebP,
        },
        _ => image::ImageOutputFormat::Png,
    };

    let mut output_buffer = Cursor::new(Vec::new());

    img.write_to(&mut output_buffer, image_format).unwrap();

    let elapsed = start.elapsed();

    ctx.send(
        CreateReply::default()
            .content(format!("processed in: {}ms", elapsed.as_millis()))
            .attachment(CreateAttachment::bytes(
                output_buffer.into_inner(),
                format!("avatar.{}", image_format_str),
            )),
    )
    .await?;

    Ok(())
}

/*fn apply_grayscale(img: &mut DynamicImage) {
    *img = img.grayscale();
}*/

/*fn apply_inverted(img: &mut DynamicImage) {
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

    *img = DynamicImage::ImageRgb8(inverted_img);
}*/

/*fn apply_sepia(img: &mut DynamicImage) {
    let mut sepia_tone_img = img.to_rgb8();

    for pixel in sepia_tone_img.pixels_mut() {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

        let tr = (0.393 * r as f32 + 0.769 * g as f32 + 0.189 * b as f32).min(255.0) as u8;
        let tg = (0.349 * r as f32 + 0.686 * g as f32 + 0.168 * b as f32).min(255.0) as u8;
        let tb = (0.272 * r as f32 + 0.534 * g as f32 + 0.131 * b as f32).min(255.0) as u8;

        *pixel = image::Rgb([tr, tg, tb]);

    }

    *img = DynamicImage::ImageRgb8(sepia_tone_img);

}

fn apply_blur(img: &mut DynamicImage, radius: f32) {
    *img = img.blur(radius);
}*/

async fn download_avatar(ctx: Context<'_>, url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!(
                        "Failed to download avatar\nstatus: {}",
                        response.status()
                    ))
                    .color(0xFF0000),
            ),
        )
        .await?;

        return Err(Box::new(PrintError(format!(
            "Error: {}",
            response.status()
        ))));
    }

    let bytes = response.bytes().await?;

    Ok(bytes.to_vec())
}
