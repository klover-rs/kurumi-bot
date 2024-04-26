use poise::serenity_prelude as serenity;
use serenity::GuildId;
use crate::Error;
use crate::db::guild_list::Database;


pub async fn guild_join(guild_id: &GuildId, is_new: bool) -> Result<(), Error> {
    if !is_new {
        return Ok(());
    }
    
    let db = Database::new().await?;

    db.create_table().await?;

    db.insert(guild_id.to_string().parse()?).await?;

    Ok(())
}

pub async fn guild_remove(guild_id: &GuildId) -> Result<(), Error> {
    let db = Database::new().await?;

    db.create_table().await?;

    db.delete(guild_id.to_string().parse()?).await?;

    Ok(())
}