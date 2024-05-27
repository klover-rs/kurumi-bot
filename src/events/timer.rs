use crate::{db::timer::Database, secrets::get_secret};
use chrono::Utc;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::task;

pub async fn check_timer() {
    let db = Arc::new(Database::new().await.unwrap());
    db.create_table().await.unwrap();

    let db_clone = Arc::clone(&db);

    task::spawn(async move {
        let database = db_clone;

        loop {
            match database.read().await {
                Ok(data) => {
                    let current_timestamp = Utc::now().timestamp();

                    for timer_recs in data {
                        if current_timestamp >= timer_recs.duration {
                            println!("Timer expired: {}", timer_recs.uid);

                            let embed = json!({
                                "title": "Timer expired",
                                "description": "the timer expired",
                                "color": 16711680,
                                "fields": [
                                    {"name": "Description", "value": timer_recs.description},
                                    {"name": "User", "value": format!("<@{}>", timer_recs.uid)}
                                ]
                            });

                            database.delete_by_id(timer_recs.id).await.unwrap();

                            if let Err(err) = send_message(
                                &timer_recs.dm_channel.to_string(),
                                &format!("<@{}>", timer_recs.uid),
                                Some(embed),
                            )
                            .await
                            {
                                println!("Failed to send message: {:?}", err);
                            }
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

async fn send_message(
    channel_id: &str,
    message: &str,
    embed: Option<Value>,
) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!(
        "https://discord.com/api/v9/channels/{}/messages",
        channel_id
    );

    let mut json_body = json!({
        "content": message
    });

    if let Some(embed_data) = embed {
        json_body["embed"] = embed_data;
    }

    let response = client
        .post(&url)
        .header(
            "Authorization",
            format!("Bot {}", get_secret("DISCORD_TOKEN")),
        )
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Failed to send message. Status code: {}", response.status());
    }

    Ok(())
}
