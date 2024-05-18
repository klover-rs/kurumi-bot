use crate::conf::utils;
use crate::db::configuration;
use crate::db::configuration::Configuration;
use crate::secrets::get_secret;
use compact_str::ToCompactString;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user: String,
    pub token: String,
    pub app_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub user: Option<String>,
    pub password: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub user: Option<HashMap<String, String>>,

    pub database: Option<DbConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub user: User,

    pub db: DbConfig,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}
impl Config {
    pub fn new() -> Self {
        let user = User::new("User".to_string());
        let config = configuration::Configuration::new();
        let db = DbConfig::new(
            "user".to_string(),
            "password".to_string(),
            "localhost".to_string(),
            0,
        );
        Self { user, config, db }
    }

    pub fn get() -> Result<Self, String> {
        let mut cf = Config::new();
        let config = match ConfigFile::get() {
            Ok(config) => config,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(user_tmp) = config.user {
            let mut user_name = String::new();
            let mut user_pass = String::new();
            let mut user_token = String::new();
            let mut user = User::new(user_name);
            for (k, v) in user_tmp {
                if k == "user" {
                    user.user = v
                } else if k == "token" {
                    user.token = v
                } else if k == "app_id" {
                    user.app_id = v
                }
            }
        }

        if let Some(db) = config.database {
            let mut db_name = String::new();
            let mut db_pass = String::new();
            let mut db_ip = String::new();
            let mut db_port: u32 = 0;

            // if let Some(name) = db.user {
            //     db_name = name
            // }
            // if let Some(pass) = db.password {
            //     db_pass = pass
            // }
            // if let Some(ip) = db.ip {
            //     db_ip = ip
            // }
            // if let Some(port) = db.port {
            //     db_port = port
            // }
            cf.db = db.clone();
        }

        if let Some(config) = config.config {
            let mut discord_conf = Configuration::new();
            for (k, v) in config {
                match k.as_str() {
                    "guild_id" => discord_conf.guild_id = v,
                    "log_channel" => discord_conf.log_channel = v,
                    "welcome_channel" => discord_conf.welcome_channel = v,
                    "mod_log_channel" => discord_conf.mod_log_channel = v,
                    "xp_channel" => discord_conf.xp_channel = v,
                    _ => {}
                }
            }
            cf.config = discord_conf;
        }

        Ok(cf)
    }
}

impl User {
    pub fn new(user: String) -> Self {
        Self {
            user,
            token: String::new(),
            app_id: String::new(),
        }
    }
}

impl DbConfig {
    pub fn new(db_user: String, db_pass: String, db_ip: String, db_port: u32) -> DbConfig {
        DbConfig {
            user: Some(db_user.to_string()),
            password: Some(db_pass),
            ip: Some(db_ip),
            port: Some(db_port),
        }
    }
}
impl ConfigFile {
    pub fn get() -> Result<Self, toml::de::Error> {
        let config = utils::get_config_file();
        Ok(toml::from_str::<ConfigFile>(
            &std::fs::read_to_string(config).unwrap(),
        )?)
    }
    pub const fn new() -> Self {
        Self {
            user: None,
            config: None,
            database: None,
        }
    }

    pub fn to_config(&self) -> Result<Config, String> {
        let file = utils::get_config_file();
        let s = match std::fs::read_to_string(file) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        let config_file = match toml::from_str::<ConfigFile>(&s) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        let mut config = Config::new();

        Ok(config)
    }

    pub fn create() -> Result<(), std::io::Error> {
        let mut config = ConfigFile::new();

        let user = HashMap::from([("user".to_string(), "user".to_string())]);
        let db_conf = HashMap::from([
            ("db_user".to_string(), "db_user".to_string()),
            ("db_pass".to_string(), "db_pass".to_string()),
        ]);
        let discord = HashMap::from([
            ("guild_id".to_string(), 0),
            ("log_channel".to_string(), 0),
            ("welcome_channel".to_string(), 0),
            ("mod_log_channel".to_string(), 0),
            ("xp_channel".to_string(), 0),
        ]);

        let db = DbConfig::new(
            "user".to_string(),
            "password".to_string(),
            "localhost".to_string(),
            0,
        );
        config.user = Some(user);
        config.config = Some(discord);
        config.database = Some(db);
        let config = toml::to_string(&config).unwrap();

        let dir = utils::get_config_dir();

        if !dir.exists() {
            utils::mk_config_dir()?;
        }
        let file = utils::get_config_file();

        std::fs::write(file, config)?;
        //
        Ok(())
    }
}

pub fn global() {
    let config = match Config::get() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    if let Ok(mut c) = CONFIG.lock() {
        *c = config;
    }
}
