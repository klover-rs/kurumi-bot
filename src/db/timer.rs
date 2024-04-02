use crate::secrets::get_secret;
use crate::Error;
use sqlx::{postgres::PgRow, PgPool, Row};
pub struct Database {
    pool: PgPool,
}

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub uid: i64,
    pub description: String,
    pub duration: i64,
    pub dm_channel: i64,
}

impl Database {
    // Initialize the connection pool
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
        // Note the change in the return type here

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS timer (
                id SERIAL PRIMARY KEY,
                uid BIGINT,
                description TEXT,
                duration BIGINT,
                dm_channel BIGINT
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert(
        &self,
        uid: i64,
        description: &str,
        time: i64,
        dm_channel: i64,
    ) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "INSERT INTO timer (uid, description, duration, dm_channel) VALUES ($1, $2, $3, $4)",
        )
        .bind(uid)
        .bind(description)
        .bind(time)
        .bind(dm_channel)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn read(&self) -> Result<Vec<Timer>, Error> {
        let mut transaction = self.pool.begin().await?;
        let rows = sqlx::query("SELECT * FROM timer")
            .fetch_all(&mut *transaction)
            .await?;
        transaction.commit().await?;
        let mut timer_records = Vec::new();

        for row in rows {
            timer_records.push(parse_timer_record(row)?);
        }

        Ok(timer_records)
    }

    pub async fn read_by_uid(&self, uid: i64) -> Result<Vec<Timer>, Error> {
        let mut timer_records = Vec::new();
        let mut transaction = self.pool.begin().await?;

        let rows = sqlx::query("SELECT * FROM timer WHERE uid = $1")
            .bind(uid)
            .fetch_all(&mut *transaction)
            .await?;

        transaction.commit().await?;

        for row in rows {
            timer_records.push(parse_timer_record(row)?);
        }

        Ok(timer_records)
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM timer WHERE id = $1")
            .bind(id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }
}

fn parse_timer_record(row: PgRow) -> Result<Timer, Error> {
    Ok(Timer {
        id: row.try_get(0)?,
        uid: row.try_get(1)?,
        description: row.try_get(2)?,
        duration: row.try_get(3)?,
        dm_channel: row.try_get(4)?,
    })
}
