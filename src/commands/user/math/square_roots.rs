use crate::{Context, Error};
use poise::{
    serenity_prelude::{self as serenity},
    CreateReply,
};
use serenity::builder::CreateEmbed;
use serenity::colours::roles::GREEN;

#[poise::command(prefix_command, slash_command)]
pub async fn square_roots(
    ctx: Context<'_>,
    #[description = "the number you want to get the square root of"] number: f64,
) -> Result<(), Error> {
    let tolerance: f64 = 1e-10;
    let mut guess: f64 = number / 2.0;

    while (guess * guess - number).abs() > tolerance {
        guess = 0.5 * (guess + number / guess);
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .description(format!("The square root of {} is {}", number, guess))
                .color(GREEN),
        ),
    )
    .await?;

    Ok(())
}
