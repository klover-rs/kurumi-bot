/*use rusqlite::{params, Result as RusqliteResult};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use r2d2::Error as R2d2Error;
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
	pool: Mutex<Pool<SqliteConnectionManager>>,
}

impl Database {
    pub fn new(database_path: &str) -> Result<Self, r2d2::Error> {
        let manager = SqliteConnectionManager::file(Path::new(database_path));
        let pool = Pool::new(manager).map_err(R2d2Error::from)?;
        Ok(Database { pool: Mutex::new(pool) })
    }

    pub fn create_table_muted(&self) -> RusqliteResult<()> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS muted (
                uid INTEGER PRIMARY KEY,
                guild_id INTEGER,
                reason TEXT,
                roles TEXT,
                duration TEXT
            )", [],

        )?;

        println!("Muted table created");
        Ok(())
    }

    pub fn insert_muted(&self, uid: i64, guild_id: i64, reason: &str, roles: Vec<u64>, duration: i64) -> RusqliteResult<()> {
        
        let mut connection = self.pool.lock().unwrap().get().unwrap();

        let roles_str = roles.iter().map(|&role| role.to_string()).collect::<Vec<String>>().join(",");

        let trans = connection.transaction()?;

        trans.execute(
            "INSERT OR REPLACE INTO muted (uid, guild_id, reason, roles, duration) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![uid, guild_id, reason, roles_str, duration],
        )?;

        trans.commit()?;

        println!("Muted inserted");
        Ok(())
    }

    pub fn read_muted(&self) -> RusqliteResult<Vec<(i64, i64, String, String, i64)>> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        let mut statement = connection.prepare("SELECT * FROM muted")?;

        let rows = statement.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })?;

        let data: Result<Vec<_>, _> = rows.collect();

        match data {
            Ok(data) => Ok(data),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Vec::new()),
            Err(err) => Err(err),
        }

    }

    pub fn delete_muted(&self, uid: i64) -> RusqliteResult<()> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        let mut statement = connection.prepare("DELETE FROM muted WHERE uid = ?1")?;

        statement.execute(params![uid])?;

        println!("Muted deleted");

        Ok(())
    }

}*/

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Error, Row};

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
            "host=localhost user=postgres password=7522",
            NoTls,
        ).expect("Failed to create connection manager");

        let pool = Pool::builder().build(manager).await.expect("Failed to build pool");
        Ok(Database { pool })
    }

    pub async fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> { // Note the change in the return type here
        let conn = self.pool.get().await?;
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

    pub async fn insert(&self, uid: i64, guild_id: i64, reason: &str, roles: Vec<u64>, duration: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;

        let roles_str = roles.iter().map(|&role| role.to_string()).collect::<Vec<String>>().join(",");

        let trans = conn.transaction().await?;

        trans.execute(
            "INSERT INTO muted (uid, guild_id, reason, roles, duration) VALUES ($1, $2, $3, $4, $5)",
            &[&uid, &guild_id, &reason, &roles_str, &duration],
        ).await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn read_muted(&self) -> Result<Vec<Muted>, Box<dyn std::error::Error>> {
        let mut muted_records = Vec::new();
        let connection = self.pool.get().await?;

        let statement = connection.prepare("SELECT * FROM muted").await?;

        let rows = connection.query(&statement, &[]).await?;

        for row in rows {
            muted_records.push(parse_muted_record(row)?);
        }

        Ok(muted_records)
    }

    pub async fn delete(&self, uid: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;

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