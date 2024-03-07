use rusqlite::{params, Result as RusqliteResult};
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

    pub fn create_table_timer(&self) -> RusqliteResult<()> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS timer (
                id INTEGER PRIMARY KEY,
                uid INTEGER,
                description TEXT,
                time INTEGER,
                dm_channel INTEGER,
                bot_message_id INTEGER
            )", [],
        )?;

        println!("Timer table created");

        Ok(())
    }

    pub fn insert_timer(&self, uid: i64, description: &str, time: i64, dm_channel: i64, bot_msg_id: i64) -> RusqliteResult<()> {
        let mut connection = self.pool.lock().unwrap().get().unwrap();


        let trans = connection.transaction()?;

        trans.execute(
            "INSERT OR REPLACE INTO timer (uid, description, time, dm_channel, bot_message_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![uid, description, time, dm_channel, bot_msg_id],
        )?;

        trans.commit()?;

        println!("Timer inserted");

        Ok(())

    }

    pub fn read_timer(&self) -> RusqliteResult<Vec<(i64, i64, String, i64, i64, i64)>> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        let mut stmt = connection.prepare("SELECT * FROM timer")?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
        })?;

        let data: Result<Vec<_>, _> = rows.collect();

        match data {
            Ok(data) => Ok(data),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Vec::new()),
            Err(err) => Err(err),
        }
    }

    pub fn read_timer_by_uid(&self, uid: i64) -> RusqliteResult<Vec<(i64, i64, String, i64, i64, i64)>> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        let mut stmt = connection.prepare("SELECT * FROM timer WHERE uid = ?1")?;

        let rows = stmt.query_map([uid], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
        })?;

        let data: Result<Vec<_>, _> = rows.collect();

        match data {
            Ok(data) => Ok(data),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Vec::new()),
            Err(err) => Err(err),
        }
    }

    pub fn delete_timer(&self, id: i64) -> RusqliteResult<()> {
        let mut connection = self.pool.lock().unwrap().get().unwrap();

        let trans = connection.transaction()?;

        trans.execute(
            "DELETE FROM timer WHERE id = ?1",

            params![id],
        )?;

        trans.commit()?;

        Ok(())
    }
}