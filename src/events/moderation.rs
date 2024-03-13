use crate::db::moderation::muted::Database;
use chrono::Utc;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use tokio::task;
use std::sync::Arc;

use crate::secrets::get_secret;

pub async fn check_mutes() {
    let db = Arc::new(Database::new("moderation.db").unwrap());
    db.create_table_muted().unwrap();

    let db_clone = Arc::clone(&db);

    task::spawn(async move {
        let database = db_clone;

        let current_timestamp = Utc::now().timestamp();


        loop {
            match database.read_muted() {
                Ok(data) => {
                    println!("data: {:?}", data);
                    for (uid, guild_id, reason, roles, duration) in data {
                        if current_timestamp >= duration {
                            println!("MUTED EXPIRED: {}", uid);
                            database.delete_muted(uid).unwrap();
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        
    });
}

async fn add_roles(roles: Vec<String>, uid: i64, guild_id: i64) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, uid);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let token = get_secret("BOT_TOKEN");

    let mut authorization_value = HeaderValue::from_str(&format!("Bot {}", token)).unwrap();
    authorization_value.set_sensitive(true);
    headers.insert(AUTHORIZATION, authorization_value);

    let body = json!({
        "roles": roles
    });

    let response = client
    .patch(&url)
    .headers(headers)
    .json(&body)
    .send()
    .await?;

    println!("Response: {:?}", response);
    Ok(())


}