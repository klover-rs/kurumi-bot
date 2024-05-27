use crate::{Context, Error};

use poise::serenity_prelude as serenity;

use poise::CreateReply;
use serenity::builder::CreateEmbed;

#[derive(Debug, poise::ChoiceParameter)]
pub enum StringChoice {
    #[name = "Rock"]
    Rock,
    #[name = "Paper"]
    Paper,
    #[name = "Scissor"]
    Scissor,
}
///Play rocker paper scissors with the bot
#[poise::command(prefix_command, slash_command)]
pub async fn rock_paper_scissors(
    ctx: Context<'_>,
    #[description = "The choice you want to choose"] choice: StringChoice,
) -> Result<(), Error> {
    let items = ["rock", "paper", "scissor"];

    let mut rng = fastrand::Rng::new();
    let index = rng.usize(..items.len());
    let computer_choice = items[index];

    let mut result = String::new();

    match choice {
        StringChoice::Rock => {
            if items[0] == computer_choice {
                result.push_str("It's a tie!");
            } else if computer_choice == "scissor" {
                result.push_str("You win lmfao :pouting_cat: ");
            } else {
                result.push_str("KURUMI wins >:3");
            }
        }
        StringChoice::Paper => {
            if items[1] == computer_choice {
                result.push_str("It's a tie!");
            } else if computer_choice == "rock" {
                result.push_str("You win lmfao :pouting_cat: ");
            } else {
                result.push_str("KURUMI wins >:3");
            }
        }
        StringChoice::Scissor => {
            if items[2] == computer_choice {
                result.push_str("It's a tie!");
            } else if computer_choice == "paper" {
                result.push_str("You win lmfao :pouting_cat: ")
            } else {
                result.push_str("KURUMI wins >:3")
            }
        }
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("RPS Game")
                .description(format!(
                    "- You entered {:?}.\n- Kurumi chose: {}\n\n**{}**",
                    choice, computer_choice, result
                ))
                .color(0xa33a0d),
        ),
    )
    .await?;
    Ok(())
}
