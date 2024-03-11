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

    pub fn create_table_logs(&self) -> RusqliteResult<()> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                msg_id INTEGER PRIMARY KEY,
                guild_id INTEGER,
                channel_id INTEGER,
                author_id INTEGER,
                content TEXT
            )", [],
            
        )?;

        println!("Logs table created");
        Ok(())
    }

    pub fn insert_log(&self, msg_id: u64, guild_id: u64, channel_id: u64, author_id: u64, content: &str) -> RusqliteResult<()> {
        let mut connection = self.pool.lock().unwrap().get().unwrap();

        let trans = connection.transaction()?;
        // Insert the new log entry
        trans.execute(
            "INSERT INTO logs (msg_id, guild_id, channel_id, author_id, content) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![msg_id, guild_id, channel_id, author_id, content],
        )?;

        // Check the number of rows in the table
        let row_count: u64 = trans.query_row("SELECT COUNT(*) FROM logs", [], |row| row.get(0)).unwrap();

        if row_count > 1000 {
            trans.execute(
                "DELETE FROM logs WHERE msg_id = (SELECT msg_id FROM logs ORDER BY msg_id ASC LIMIT 1)",
                [],
            )?;
        }

        trans.commit()?;

        Ok(())
    }

    pub fn update_log_content(&self, msg_id: u64, new_content: &str) -> RusqliteResult<()> {
        let mut connection = self.pool.lock().unwrap().get().unwrap();

        let trans = connection.transaction()?;

        trans.execute(
            "UPDATE logs SET content = ?1 WHERE msg_id = ?2",
            params![new_content, msg_id],
        )?;

        trans.commit()?;

        Ok(())
    }

    pub fn read_logs_by_id(&self, msg_id: u64) -> RusqliteResult<Vec<(u64, u64, u64, u64, String)>> {
        let connection = self.pool.lock().unwrap().get().unwrap();

        let mut statement = connection.prepare("SELECT * FROM logs WHERE msg_id = ?1")?;

        let rows = statement.query_map(params![msg_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })?;

        let data: Result<Vec<_>, _> = rows.collect();

        match data {
            Ok(data) => Ok(data),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Vec::new()),
            Err(err) => Err(err),
        }
    }

}