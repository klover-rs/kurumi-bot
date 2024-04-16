use crate::{secrets::get_secret, PrintError};
use serde::{Deserialize, Serialize};
use crate::Error;
use sqlx::{postgres::PgRow, PgPool, Row};
pub struct Database {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub guild_id: i64,
    pub log_channel: i64,
    pub mod_log_channel: i64,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let url = format!(
            "postgresql://postgres:{}@localhost:5432/{}",
            get_secret("DB_PW"),
            get_secret("DB_NAME")
        );
        let url = url.as_str();
        let pool = sqlx::postgres::PgPool::connect(url).await?;
        Ok(Self { pool })
        /* postgresql://postgres:7522@localhost:5432/kurumi_shit"  */
    }

    pub async fn create_table(&self) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS configuration (
            guild_id BIGINT PRIMARY KEY,
            log_channel BIGINT DEFAULT 0,
            mod_log_channel BIGINT DEFAULT 0
        )")
        .execute(&self.pool)
        .await?;
        Ok(())

    }

    pub async fn insert(
        &self,
        guild_id: i64,
        log_channel: Option<i64>,
        mod_log_channel: Option<i64>
    ) -> Result<(), Error> {
        let trans = self.pool.begin().await?;

        

        match (log_channel, mod_log_channel) {
            (Some(log_channel), Some(mod_log_channel)) => {
                sqlx::query("INSERT INTO configuration (guild_id, log_channel, mod_log_channel) VALUES ($1, $2, $3)")
                    .bind(guild_id)
                    .bind(log_channel)
                    .bind(mod_log_channel)
                    .execute(&self.pool)
                    .await?;
                trans.commit().await?;
            }
            (Some(log_channel), None) => {
                sqlx::query("INSERT INTO configuration (guild_id, log_channel) VALUES ($1, $2)")
                    .bind(guild_id)
                    .bind(log_channel)
                    .execute(&self.pool)
                    .await?;

                trans.commit().await?;
            }
            (None, Some(mod_log_channel)) => {
                sqlx::query("INSERT INTO configuration (guild_id, mod_log_channel) VALUES ($1, $2)")
                    .bind(guild_id)
                    .bind(mod_log_channel)
                    .execute(&self.pool)
                    .await?;

                trans.commit().await?;
            }
            (None, None) => {
                return Err(Box::new(PrintError("No log_channel or mod_log_channel provided".to_string())));
            }
        }

        Ok(())
    }

    pub async fn update(
        &self,
        guild_id: i64,
        log_channel: Option<i64>,
        mod_log_channel: Option<i64>
    ) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;

        println!("INSERTING:\n{:?}\n{:?}", log_channel, mod_log_channel);

        match (log_channel, mod_log_channel) {
            (Some(log_channel), Some(mod_log_channel)) => {
                sqlx::query("UPDATE configuration SET log_channel = $1, mod_log_channel = $2 WHERE guild_id = $3")
                    .bind(log_channel)
                    .bind(mod_log_channel)
                    .bind(guild_id)
                    .execute(&mut *trans)
                    .await?;
            }
            (Some(log_channel), None) => {
                sqlx::query("UPDATE configuration SET log_channel = $1 WHERE guild_id = $2")
                    .bind(log_channel)
                    .bind(guild_id)
                    .execute(&mut *trans)
                    .await?;
            }
            (None, Some(mod_log_channel)) => {
                sqlx::query("UPDATE configuration SET mod_log_channel = $1 WHERE guild_id = $2")
                    .bind(mod_log_channel)
                    .bind(guild_id)
                    .execute(&mut *trans)
                    .await?;
            }
            (None, None) => {
                return Err(Box::new(PrintError("No log_channel or mod_log_channel provided".to_string())));
            }
        }

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
        mod_log_channel: row.try_get(2)?
    })
}