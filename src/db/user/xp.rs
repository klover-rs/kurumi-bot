use crate::{secrets::get_secret, PrintError};

use serde::{Deserialize, Serialize};
use crate::Error;
use sqlx::{postgres::PgRow, PgPool, Row};
pub struct Database {
    pool: PgPool,
}

pub struct Xp {
    pub uid: i64,
    pub guild_id: i64,
    pub xp: i64,
    pub rank: i64,
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
        sqlx::query("CREATE TABLE IF NOT EXISTS xp (
            guild_id BIGINT PRIMARY KEY,
            uid BIGINT,
            xp BIGINT,
            rank BIGINT DEFAULT 0
        )")
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert(&self, uid: i64, guild_id: i64, xp: i64) -> Result<(), Error> {
        let mut trans = self.pool.begin().await?;

        sqlx::query("INSERT INTO xp (guild_id, uid, xp) VALUES ($1, $2, $3)")
            .bind(guild_id)
            .bind(uid)
            .bind(xp)
            .execute(&mut *trans)
            .await?;

        trans.commit().await?;

        Ok(())
    }

    pub async fn update(&self, uid: i64, guild_id: i64, xp: i64, rank: i64) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        
        // Define the SQL UPDATE query with placeholders
        let query = "UPDATE xp SET xp = $1, rank = $2 WHERE guild_id = $3 AND uid = $4";
        
        // Execute the SQL query with bound parameters
        sqlx::query(query)
            .bind(xp)
            .bind(rank)
            .bind(guild_id)
            .bind(uid)
            .execute(&mut *transaction)
            .await?;
        
        // Commit the transaction
        transaction.commit().await?;
        
        Ok(())
    }
    
    pub async fn read(&self, uid: i64, guild_id: i64) -> Result<Vec<Xp>, Error> {
        let mut xp_record = Vec::new();

        let rows = sqlx::query("SELECT * FROM xp WHERE guild_id = $1 AND uid = $2")
            .bind(guild_id)
            .bind(uid)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            xp_record.push(parse_xp_record(row)?);
        }

        Ok(xp_record)
    }
}

fn parse_xp_record(row: PgRow) -> Result<Xp, Error> {
    Ok(Xp {
        uid: row.try_get(0)?,
        guild_id: row.try_get(1)?,
        xp: row.try_get(2)?,
        rank: row.try_get(3)?,
    })
}