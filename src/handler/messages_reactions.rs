use poise::serenity_prelude as serenity;

use serenity::model::channel::ReactionType;
use regex::Regex;
use crate::Error;


use serenity::Message;

pub async fn message_reactions(msg: &Message, ctx: &serenity::Context) -> Result<(), Error> {
    println!("message: {}", msg.content);

    let regex_pattern = Regex::new(r"\bfreddy\b").unwrap();

    if regex_pattern.is_match(&msg.content) {
       
        let custom_emote = ReactionType::Custom {
            animated: false,
            id: serenity::EmojiId::from(1216804276278394991),
            name: None, 
        };
        
        msg.react(&ctx, custom_emote).await.unwrap();
    }


    Ok(())
}