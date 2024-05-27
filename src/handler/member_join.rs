use crate::Error;

use poise::serenity_prelude as serenity;
use serenity::ChannelId;

use serenity::builder::CreateMessage;

pub async fn member_join(member: &serenity::Member, ctx: &serenity::Context) -> Result<(), Error> {
    let channel = ChannelId::from(1224112266446639217);

    channel
        .send_message(
            ctx,
            CreateMessage::default().content(format!("Welcome {}!", member)),
        )
        .await?;

    Ok(())
}
