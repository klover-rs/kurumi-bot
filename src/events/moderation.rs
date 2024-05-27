use crate::db::moderation::muted::Database;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use tokio::task;

use crate::secrets::get_secret;

pub async fn check_mutes() {
    let db = Arc::new(Database::new().await.unwrap());
    db.create_table().await.unwrap();

    let db_clone = Arc::clone(&db);
    task::spawn(async move {
        let database = db_clone;

        loop {
            let current_timestamp = Utc::now().timestamp();

            match database.read_muted().await {
                Ok(data) => {
                    for muted_record in data {
                        if current_timestamp >= muted_record.duration {
                            println!("MUTED EXPIRED: {}", muted_record.uid);
                            database.delete(muted_record.uid).await.unwrap();
                            let roles_vec: Vec<&str> = muted_record.roles.split(',').collect();
                            add_roles(roles_vec, muted_record.uid, muted_record.guild_id)
                                .await
                                .unwrap();
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
}

async fn add_roles(roles: Vec<&str>, uid: i64, guild_id: i64) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!(
        "https://discord.com/api/v9/guilds/{}/members/{}",
        guild_id, uid
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let token = get_secret("DISCORD_TOKEN");

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
