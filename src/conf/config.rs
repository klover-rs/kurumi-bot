use crate::conf::utils;
use crate::db::configuration;
use crate::db::configuration::Configuration;
use crate::secrets::get_secret;
use crate::Asset;
use compact_str::ToCompactString;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotConfig {
    pub name: Option<String>,
    pub id: Option<String>,
    pub app_id: Option<String>,
    pub token: Option<String>,
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
    pub bot: Option<BotConfig>,
    pub database: Option<DbConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub db: DbConfig,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}
impl Config {
    pub fn new() -> Self {
        let db = DbConfig::new(
            "user".to_string(),
            "password".to_string(),
            "localhost".to_string(),
            0,
        );
        let bot = BotConfig {
            name: Some(String::from("kurumi-bot")),
            id: Some(String::from("bot_id")),
            app_id: None,
            token: Some(String::from("DISCORD_TOKEN")),
        };
        Self { db, bot }
    }

    pub fn get() -> Result<Self, String> {
        let mut cf = Config::new();
        let config = match ConfigFile::get() {
            Ok(config) => config,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(bot) = config.bot {
            cf.bot = bot;
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

        Ok(cf)
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
            bot: None,
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

        let bot = BotConfig {
            name: Some(String::from("kurumi-bot")),
            id: Some(String::from("bot_id")),
        };

        config.bot = Some(bot);
        config.database = Some(db);
        let config = toml::to_string(&config).unwrap();

        let dir = utils::get_config_dir();

        if !dir.exists() {
            utils::mk_config_dir()?;
        }
        let file = utils::get_config_file();

        let cf = Asset::get("config.toml").unwrap();
        let config = std::str::from_utf8(cf.data.as_ref()).unwrap();
        // println!("{}", config);
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
