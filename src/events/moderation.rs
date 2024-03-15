use crate::db::moderation::muted::Database;
use chrono::Utc;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use tokio::task;
use std::sync::Arc;
use tokio::task::LocalSet;


use crate::secrets::get_secret;

pub async fn check_mutes() -> Result<(), Box<dyn std::error::Error + Send>> {
    let db = Arc::new(Database::new().await.unwrap());
    db.create_table().await.unwrap();

    let db_clone = Arc::clone(&db);

    std::thread::spawn(move || {
        let database = db_clone;

        let rt = tokio::runtime::Runtime::new().unwrap();

        loop {
            let current_timestamp = Utc::now().timestamp();

            rt.block_on(async {
                match database.read_muted().await {
                    Ok(data) => {
                        println!("data: {:?}", data);
    
                        for muted_record in data {
                            if current_timestamp >= muted_record.duration {
                                println!("MUTED EXPIRED: {}", muted_record.uid);
                                database.delete(muted_record.uid).await.unwrap();
                                let roles_vec: Vec<&str> = muted_record.roles.split(',').collect();
                                add_roles(roles_vec, muted_record.uid, muted_record.guild_id).await.unwrap();
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        

                    }
                }
            });

                
            
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    Ok(())
}


async fn add_roles(roles: Vec<&str>, uid: i64, guild_id: i64) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!("https://discord.com/api/v9/guilds/{}/members/{}", guild_id, uid);

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