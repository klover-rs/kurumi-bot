use crate::{Context, Error};
use std::error::Error as stdError;
use std::fmt;


use poise::CreateReply;

use reqwest::Client;

use serde_json::json;

use crate::secrets::get_secret;

/// Shows the bot's latency
#[poise::command(prefix_command, slash_command)]
pub async fn authorize(
    ctx: Context<'_>,
    #[description = "Enter the auth code you got from the dashboard"] auth_code: u32
) -> Result<(), Error> {

    let api_url = get_secret("API_URL");

    let user_id = ctx.author().id.to_string();


    let response = send_authorize_request(&api_url, auth_code, &user_id).await;

    println!("hi");


    let response = match response {
        Ok(body) => {
            println!("Response: {}", body);

            let json: serde_json::Value = serde_json::from_str(&body).unwrap();
            json["message"].as_str().unwrap().to_string()

        }
        Err(e) => {
            println!("Error: {}", e);

            let json: serde_json::Value = serde_json::from_str(&e.to_string()).unwrap();
            json["message"].as_str().unwrap().to_string()
        }
    };

    ctx.send(CreateReply::default().content(response).ephemeral(true)).await?;


    Ok(())
}

#[derive(Debug)]
struct HttpError(String);

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl stdError for HttpError {}


pub async fn send_authorize_request(api_url: &str, auth_code: u32, user_id: &str) -> Result<String, Box<dyn std::error::Error + Send>> {
    let api_url = format!("{}auth_code/validate", api_url);

    // Send the request using the Reqwest client
    let client = Client::new();
    let response = match client
        .post(&api_url)
        .json(&json!({
            "code": auth_code,
            "discord_uid": user_id
        }))
        .send()
        .await {
            Ok(response) => response,
            Err(err) => return Err(Box::new(HttpError(format!("Reqwest error: {}", err)))), // Wrap the string error message with HttpError
        };

    // Check if the response was successful (status code 2xx)
    if response.status().is_success() {
        // Read and return the response body
        let body = match response.text().await {
            Ok(body) => body,
            Err(err) => return Err(Box::new(HttpError(format!("Reqwest error: {}", err)))),
        };
        println!("Response: {}", body);
        Ok(body)
    } else {
        // If the response was not successful, return an error with the status code
        Err(Box::new(HttpError(format!("Request failed with status code: {}", response.status()))))
    }
}