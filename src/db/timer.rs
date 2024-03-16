
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Error, Row};
use crate::secrets::get_secret;
pub struct Database {
    pool: Pool<PostgresConnectionManager<NoTls>>,
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
    pub async fn new() -> Result<Self, Error> {
        let manager = PostgresConnectionManager::new_from_stringlike(
            format!("host=localhost user=postgres password={}", get_secret("DB_PW")),
            NoTls,
        ).expect("Failed to create connection manager");

        let pool = Pool::builder().build(manager).await.expect("Failed to build pool");
        Ok(Database { pool })
    }

    pub async fn create_table(&self) -> Result<(), Error> { // Note the change in the return type here
        let conn = self.pool.get().await.unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS timer (
                id SERIAL PRIMARY KEY,
                uid BIGINT,
                description TEXT,
                duration BIGINT,
                dm_channel BIGINT
            )",
            &[],
        ).await?;
        Ok(())
    }

    pub async fn insert(&self, uid: i64, description: &str, time: i64, dm_channel: i64) -> Result<(), Error> {
        let mut conn = self.pool.get().await.unwrap();

        let trans = conn.transaction().await?;

        trans.execute(
            "INSERT INTO timer (uid, description, duration, dm_channel) VALUES ($1, $2, $3, $4)",
            &[&uid, &description, &time, &dm_channel],
        ).await?;

        trans.commit().await?;

        Ok(())
    
    }

    pub async fn read(&self) -> Result<Vec<Timer>, Error> {
        let mut timer_records = Vec::new();

        let conn = self.pool.get().await.unwrap();

        let stmt = conn.prepare("SELECT * FROM timer").await?;

        let rows = conn.query(&stmt, &[]).await?;

        for row in rows {
            timer_records.push(parse_timer_record(row)?);
        }

        Ok(timer_records)
    }

    pub async fn read_by_uid(&self, uid: i64) -> Result<Vec<Timer>, Error> {
        let mut timer_records = Vec::new();

        let conn = self.pool.get().await.unwrap();

        let stmt = conn.prepare("SELECT * FROM timer WHERE uid = $1").await?;

        let rows = conn.query(&stmt, &[&uid]).await?;

        for row in rows {
            timer_records.push(parse_timer_record(row)?);
        }

        Ok(timer_records)

    }

    pub async fn delete_by_id(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.pool.get().await.unwrap();

        let trans = conn.transaction().await?;

        trans.execute("DELETE FROM timer WHERE id = $1", &[&id]).await?;

        trans.commit().await?;
        Ok(())
    }
}   

fn parse_timer_record(row: Row) -> Result<Timer, Error> {
    Ok(Timer {
        id: row.try_get(0)?,
        uid: row.try_get(1)?,
        description: row.try_get(2)?,
        duration: row.try_get(3)?,
        dm_channel: row.try_get(4)?,
    })

}