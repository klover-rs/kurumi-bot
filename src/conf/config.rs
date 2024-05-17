use crate::conf::utils;
use crate::db::configuration;
use crate::secrets::get_secret;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct User {
    pub user: String,
}

pub struct DbConfig {
    pub db_user: String,
    pub db_pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub user: Option<HashMap<String, String>>,
    pub config: Option<HashMap<String, String>>,
    pub database: Option<HashMap<String, String>>,
}

pub struct Config {
    pub user: User,
    pub config: configuration::Configuration,
    pub db: DbConfig,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
impl Config {
    pub fn new() -> Self {
        let user = User::new("User".to_string());
        let config = configuration::Configuration::new();
        let db = DbConfig::new("user".to_string(), "password".to_string());
        Self { user, config, db }
    }

    pub fn get() -> Result<Self, String> {
        let config = match ConfigFile::get() {
            Ok(config) => config,
            Err(e) => return Err(e.to_string()),
        };
        let mut user_name = String::new();
        let mut user_pass = String::new();
        if let Some(user) = config.user {
            for (k, v) in user {
                if k == "user" {
                    user_name = v.clone()
                }
                if k == "password" {
                    user_pass = v.clone()
                }
            }
        }

        let mut config = Config::new();
        config.user.user = user_name;
        Ok(config)
    }
}

impl User {
    pub fn new(user: String) -> Self {
        Self { user }
    }
}

impl DbConfig {
    pub fn new(db_user: String, db_pass: String) -> DbConfig {
        DbConfig { db_user, db_pass }
    }
}
impl ConfigFile {
    pub fn get() -> Result<Self, toml::de::Error> {
        let config = utils::get_config_file();
        Ok(toml::from_str::<ConfigFile>(
            &std::fs::read_to_string(config).unwrap(),
        )?)
    }
}
