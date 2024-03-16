use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Error, Row};
use crate::secrets::get_secret;
pub struct Database {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}


#[derive(Debug)]
pub struct Muted {
    pub uid: i64,
    pub guild_id: i64,
    pub reason: String,
    pub roles: String,
    pub duration: i64,
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
            "CREATE TABLE IF NOT EXISTS muted (
                uid BIGINT PRIMARY KEY,
                guild_id BIGINT,
                reason TEXT,
                roles TEXT,
                duration BIGINT
            )",
            &[],
        ).await?;
        Ok(())
    }

    pub async fn insert(&self, uid: i64, guild_id: i64, reason: &str, roles: Vec<u64>, duration: i64) -> Result<(), Error> {
        let mut conn = self.pool.get().await.unwrap();

        let roles_str = roles.iter().map(|&role| role.to_string()).collect::<Vec<String>>().join(",");

        let trans = conn.transaction().await?;

        trans.execute(
            "INSERT INTO muted (uid, guild_id, reason, roles, duration) VALUES ($1, $2, $3, $4, $5)",
            &[&uid, &guild_id, &reason, &roles_str, &duration],
        ).await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn read_muted(&self) -> Result<Vec<Muted>, Error> {
        let mut muted_records = Vec::new();
        let connection = self.pool.get().await.unwrap();

        let statement = connection.prepare("SELECT * FROM muted").await?;

        let rows = connection.query(&statement, &[]).await?;

        for row in rows {
            muted_records.push(parse_muted_record(row)?);
        }

        Ok(muted_records)
    }

    pub async fn read_muted_by_uid(&self, uid: i64) -> Result<Vec<Muted>, Error> {
        let connection = self.pool.get().await.unwrap();

        let statement = connection.prepare("SELECT * FROM muted WHERE uid = $1").await?;

        let rows = connection.query(&statement, &[&uid]).await?;

        let mut muted_records = Vec::new();

        for row in rows {
            muted_records.push(parse_muted_record(row)?);
        }

        Ok(muted_records)

    }

    pub async fn delete(&self, uid: i64) -> Result<(), Error> {
        let mut conn = self.pool.get().await.unwrap();

        let trans = conn.transaction().await?;

        trans.execute("DELETE FROM muted WHERE uid = $1", &[&uid]).await?;

        trans.commit().await?;
        Ok(())
    }

}

fn parse_muted_record(row: Row) -> Result<Muted, Error> {
    Ok(Muted {
        uid: row.try_get(0)?,
        guild_id: row.try_get(1)?,
        reason: row.try_get(2)?,
        roles: row.try_get(3)?,
        duration: row.try_get(4)?,
    })
}