use crate::{secrets::get_secret, PrintError};
use serde::{Deserialize, Serialize};
use crate::Error;
use sqlx::{postgres::PgRow, PgPool, Row};
pub struct Database {
    pool: PgPool,
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
    }

    pub async fn create_table(&self) -> Result<(), Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS guild_list (
            guild_id BIGINT PRIMARY KEY
        )")
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert(&self, guild_id: i64) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;
        sqlx::query("INSERT INTO guild_list (guild_id) VALUES ($1)")
            .bind(guild_id)
            .execute(&mut *trans)
            .await?;
        trans.commit().await?;
        Ok(())
    }

    pub async fn delete(&self, guild_id: i64) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;
        sqlx::query("DELETE FROM guild_list WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&mut *trans)
            .await?;
        trans.commit().await?;
        Ok(())
    }



}