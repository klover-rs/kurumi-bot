use crate::{db::user::xp, Error};

use poise::serenity_prelude as serenity;
use crate::db::user::xp::Database;

pub async fn handle_xp(
    message: &serenity::Message
) -> Result<(), Error> {

    let guild_id = match message.guild_id {
        Some(guild) => guild,
        None => return Ok(())
    };
    let author = message.author.id;
    let message_len = message.content.len();
    
    xp_logic(author.try_into()?, guild_id.try_into()?, message_len).await?;

    Ok(())
} 

async fn xp_logic(uid: i64, guild_id: i64, message_len: usize) -> Result<(), Error> {
    let db = Database::new().await?;

    db.create_table().await?;

    let xp_record = db.read(uid, guild_id).await?;
    if xp_record.is_empty() {
        let xp = calculate_xp(0, 0, message_len);
        db.insert(uid, guild_id, xp.0).await?;
    } else {
        let (xp, rank) = calculate_xp(xp_record[0].xp, xp_record[0].rank, message_len);
        println!("XP: {}", xp);
        db.update(uid, guild_id, xp, rank).await?;
    }

    Ok(())
}

fn calculate_xp(xp: i64, rank: i64, message_len: usize) -> (i64, i64) {
    let req_xp_to_lvl_up = 200 + (250 * rank);

    let xp_per_msg: f64 = 25.0;

    let f_message_len = message_len as f64;

    let mut xp_gain: i64 = 0;
    
    if message_len < 10 {
        xp_gain = 25;
    } else  {
        xp_gain = (xp_per_msg * (f_message_len / 100.0)).round() as i64;
    };

    if xp_gain > 50 {
        xp_gain = 50
    }

    let new_rank; 
    if xp >= req_xp_to_lvl_up {
        new_rank = rank + 1;
    } else {
        new_rank = rank;
    }

    (xp + xp_gain, new_rank)
}