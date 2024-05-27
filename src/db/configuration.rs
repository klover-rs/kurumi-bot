use crate::secrets::get_secret;
use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};
pub struct Database {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub guild_id: i64,
    pub log_channel: i64,
    pub mod_log_channel: i64,
    pub welcome_channel: i64,
    pub xp_channel: i64,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            guild_id: 0,
            log_channel: 0,
            mod_log_channel: 0,
            welcome_channel: 0,
            xp_channel: 0,
        }
    }
}

use crate::conf::config::CONFIG;
impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let mut config = crate::conf::config::Config::new();
        if let Ok(c) = CONFIG.lock() {
            config = c.clone();
        }
        let pass = config.db.password.unwrap_or("password".to_string());
        let user = config.db.user.unwrap_or("postgres".to_string());

        let ip = config.db.ip.unwrap_or("localhost".to_string());
        let port = config.db.port.unwrap_or(5432);
        // let account = config.config.user;
        let db_name = "kurumi-bot";
        let url = format!("postgresql://{user}:{pass}@{ip}:{port}/{db_name}",);
        let url = url.as_str();
        let pool = sqlx::postgres::PgPool::connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn create_table(&self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS configuration (
            guild_id BIGINT PRIMARY KEY,
            log_channel BIGINT DEFAULT 0,
            mod_log_channel BIGINT DEFAULT 0,
            welcome_channel BIGINT DEFAULT 0,
            xp_channel BIGINT DEFAULT 0
        )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert(
        &self,
        guild_id: i64,
        log_channel: Option<i64>,
        mod_log_channel: Option<i64>,
        welcome_channel: Option<i64>,
        xp_channel: Option<i64>,
    ) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;

        let log_channel = log_channel.unwrap_or(0);
        let mod_log_channel = mod_log_channel.unwrap_or(0);
        let welcome_channel = welcome_channel.unwrap_or(0);
        let xp_channel = xp_channel.unwrap_or(0);

        sqlx::query("INSERT INTO configuration (guild_id, log_channel, mod_log_channel, welcome_channel, xp_channel) VALUES ($1, $2, $3, $4, $5)")
            .bind(guild_id)
            .bind(log_channel)
            .bind(mod_log_channel)
            .bind(welcome_channel)
            .bind(xp_channel)
            .execute(&mut *trans)
            .await?;

        trans.commit().await?;

        Ok(())
    }

    pub async fn update(
        &self,
        guild_id: i64,
        log_channel: Option<i64>,
        mod_log_channel: Option<i64>,
        welcome_channel: Option<i64>,
        xp_channel: Option<i64>,
    ) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;

        let log_channel = log_channel.unwrap_or(0);
        let mod_log_channel = mod_log_channel.unwrap_or(0);
        let welcome_channel = welcome_channel.unwrap_or(0);
        let xp_channel = xp_channel.unwrap_or(0);

        sqlx::query("UPDATE configuration SET log_channel = $1, mod_log_channel = $2, welcome_channel = $3, xp_channel = $4 WHERE guild_id = $5")
            .bind(log_channel)
            .bind(mod_log_channel)
            .bind(welcome_channel)
            .bind(xp_channel)   
            .bind(guild_id)
            .execute(&mut *trans)
            .await?;

        trans.commit().await?;

        Ok(())
    }

    pub async fn read_by_guild_id(&self, guild_id: i64) -> Result<Vec<Configuration>, Error> {
        let mut configuration_record = Vec::new();
        let rows = sqlx::query("SELECT * FROM configuration WHERE guild_id = $1")
            .bind(guild_id)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            configuration_record.push(parse_configuration_record(row).unwrap());
        }

        Ok(configuration_record)
    }

    pub async fn clear_by_guild_id(&self, guild_id: i64) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;
        sqlx::query("DELETE FROM configuration WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&mut *trans)
            .await?;
        trans.commit().await?;
        Ok(())
    }
}

fn parse_configuration_record(row: PgRow) -> Result<Configuration, Error> {
    Ok(Configuration {
        guild_id: row.try_get(0)?,
        log_channel: row.try_get(1)?,
        mod_log_channel: row.try_get(2)?,
        welcome_channel: row.try_get(3)?,
        xp_channel: row.try_get(4)?,
    })
}
