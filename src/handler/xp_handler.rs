use crate::Error;

use poise::serenity_prelude as serenity;
use crate::db::user::xp::Database;
use crate::db::configuration::Database as ConfigDatabase;
use serenity::ChannelId;
use serenity::CreateMessage;
use serenity::CreateEmbed;

pub async fn handle_xp(
    message: &serenity::Message,
    ctx: &serenity::Context
) -> Result<(), Error> {

    if message.author.bot {
        return Ok(());
    }

    let guild_id = match message.guild_id {
        Some(guild) => guild,
        None => return Ok(())
    };
    let author = message.author.id;
    let message_len = message.content.len();
    
    xp_logic(ctx, author.try_into()?, guild_id.try_into()?, message_len).await?;

    Ok(())
} 

async fn xp_logic(ctx: &serenity::Context,uid: i64, guild_id: i64, message_len: usize) -> Result<(), Error> {
    let db = Database::new().await?;
    let config_db = ConfigDatabase::new().await?;

    config_db.create_table().await?;

    db.create_table().await?;

    let config = match config_db.read_by_guild_id(guild_id).await {
        Ok(config) => {
            if config.is_empty() {
                println!("config not found\n--------------------------------");
                return Ok(());
            }
            config
        },
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        }
    };


    let xp_record = db.read(uid, guild_id).await?;
    if xp_record.is_empty() {
        let (xp, _, xp_in_this_rank) = calculate_xp(0, 0, 0, message_len);
        db.insert(uid, guild_id, xp, xp_in_this_rank).await?;
    } else {
        let (xp, rank, xp_in_this_rank) = calculate_xp(xp_record[0].xp, xp_record[0].rank, xp_record[0].xp_in_this_rank, message_len);

        if rank > xp_record[0].rank {
            let xp_channel = ChannelId::from(config[0].xp_channel as u64);
            let lvl_up_embed = CreateEmbed::default().title("Rank up!").description(format!("You are now level: {}", rank)).color(0x00FF00);

            xp_channel.send_message(ctx, CreateMessage::default().content(format!("<@{}>", xp_record[0].uid)).embed(lvl_up_embed)).await?;
        }

        println!("XP: {}", xp);
        println!("Rank: {}", rank);
        db.update(uid, guild_id, xp, xp_in_this_rank, rank).await?;
    }

    Ok(())
}

#[derive(Debug)]
struct UserXp {
    pub rank: i64,
    pub msg_len: i64,
    pub xp_per_msg: i64,
    pub xp: i64,
    pub required_xp_for_next_level: i64,
    pub remaining_xp_to_next_level: i64,
    pub xp_in_this_rank: i64,
    pub has_level_up: bool,
}

fn calculate_xp(
    current_xp: i64,
    current_rank: i64,
    current_xp_in_rank: i64,
    message_len: usize
) -> (i64, i64, i64) {

    let mut user_xp = UserXp {
        rank: current_rank,
        msg_len: message_len as i64,
        xp: current_xp,
        xp_per_msg: 25,
        required_xp_for_next_level: 750 + 250 * current_rank,
        remaining_xp_to_next_level: 0,
        xp_in_this_rank: current_xp_in_rank,
        has_level_up: false,
    };

    user_xp.xp += user_xp.xp_per_msg;
    user_xp.xp_in_this_rank += user_xp.xp_per_msg;

    if user_xp.xp_in_this_rank >= user_xp.required_xp_for_next_level {
        user_xp.has_level_up = true;
        if user_xp.has_level_up {
            println!("user has leveled up!");
            user_xp.rank += 1;
            user_xp.remaining_xp_to_next_level = 0;
            user_xp.xp_in_this_rank = 0; // Reset XP gained within the current rank
            user_xp.has_level_up = false;
        }
    }

    user_xp.remaining_xp_to_next_level = user_xp.required_xp_for_next_level - user_xp.xp_in_this_rank;

    println!("user_xp: {:?}", user_xp);
    
    (user_xp.xp, user_xp.rank, user_xp.xp_in_this_rank)
    
    /*let req_xp_to_lvl_up = 1000 + 250 * rank;

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

    (xp + xp_gain, new_rank)*/
}
