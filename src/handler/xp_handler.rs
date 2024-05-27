use crate::Error;

use crate::db::configuration::Database as ConfigDatabase;
use crate::db::user::xp::Database;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;
use poise::serenity_prelude::Member;
use poise::serenity_prelude::RoleId;
use poise::serenity_prelude::UserId;
use serenity::ChannelId;
use serenity::CreateEmbed;
use serenity::CreateMessage;

pub async fn handle_xp(message: &serenity::Message, ctx: &serenity::Context) -> Result<(), Error> {
    if message.author.bot {
        return Ok(());
    }

    let guild_id = match message.guild_id {
        Some(guild) => guild,
        None => return Ok(()),
    };
    let author = message.author.id;
    let message_len = message.content.len();

    xp_logic(ctx, author.try_into()?, guild_id.try_into()?, message_len).await?;

    Ok(())
}

async fn xp_logic(
    ctx: &serenity::Context,
    uid: i64,
    guild_id: i64,
    message_len: usize,
) -> Result<(), Error> {
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
        }
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
        let (xp, rank, xp_in_this_rank) = calculate_xp(
            xp_record[0].xp,
            xp_record[0].rank,
            xp_record[0].xp_in_this_rank,
            message_len,
        );

        if rank > xp_record[0].rank {
            let xp_channel = ChannelId::from(config[0].xp_channel as u64);
            let lvl_up_embed = CreateEmbed::default()
                .title("Rank up!")
                .description(format!("You are now level: {}", rank))
                .color(0x00FF00);

            xp_channel
                .send_message(
                    ctx,
                    CreateMessage::default()
                        .content(format!("<@{}>", xp_record[0].uid))
                        .embed(lvl_up_embed),
                )
                .await?;

            let level_roles_str = db.read_level_roles(guild_id).await?;

            let level_roles = deserialize_roles_str(&level_roles_str)?;

            for (ordering_number, role_id) in level_roles {
                if ordering_number == rank as i32 {
                    let role = RoleId::from(role_id as u64);
                    let user = get_member(
                        ctx,
                        GuildId::from(guild_id as u64),
                        UserId::from(uid as u64),
                    )
                    .await
                    .unwrap();

                    user.add_role(ctx, role).await?;
                }
            }
        }

        println!("XP: {}", xp);
        println!("Rank: {}", rank);
        db.update(uid, guild_id, xp, xp_in_this_rank, rank).await?;
    }

    Ok(())
}

async fn get_member(ctx: &serenity::Context, guild_id: GuildId, user_id: UserId) -> Option<Member> {
    if let Some(member) = guild_id.member(&ctx, user_id).await.ok() {
        Some(member)
    } else {
        guild_id.member(&ctx, user_id).await.ok()
    }
}

fn deserialize_roles_str(roles_str: &str) -> Result<Vec<(i32, i64)>, Error> {
    let mut roles: Vec<(i32, i64)> = Vec::new();

    for (_, role) in roles_str.split(",").enumerate() {
        let parts: Vec<&str> = role.split('=').collect();

        let ordering_number = parts[0].parse::<i32>();
        let role_id = parts[1].parse::<i64>();

        roles.push((ordering_number?, role_id?));
    }

    Ok(roles)
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
    message_len: usize,
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

    if user_xp.msg_len < 10 {
        user_xp.xp += user_xp.xp_per_msg;
    } else {
        user_xp.xp += user_xp.xp_per_msg * (user_xp.msg_len / 100)
    }

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

    user_xp.remaining_xp_to_next_level =
        user_xp.required_xp_for_next_level - user_xp.xp_in_this_rank;

    println!("user_xp: {:?}", user_xp);

    (user_xp.xp, user_xp.rank, user_xp.xp_in_this_rank)
}
