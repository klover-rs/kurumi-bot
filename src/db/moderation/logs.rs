use crate::secrets::get_secret;
use crate::Error;

use sqlx::{Row, Transaction};
pub struct DatabaseMsgLogs {
    pool: sqlx::PgPool,
}
#[derive(Debug, Clone)]
pub struct MsgLogs {
    pub msg_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub author_id: i64,
    pub contents: String,
    pub attachments: String,
}
impl DatabaseMsgLogs {
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

    pub async fn create_table_msg_logs(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS msg_logs (
                msg_id BIGINT  PRIMARY KEY,
                guild_id BIGINT,
                channel_id BIGINT,
                author_id BIGINT,
                contents TEXT,
                attachments TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        println!("Created msg_logs table");
        Ok(())
    }
    pub async fn create_table_deleted_msgs(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS deleted_msgs (
                msg_id BIGINT  PRIMARY KEY,
                guild_id BIGINT,
                channel_id BIGINT,
                author_id BIGINT,
                contents TEXT,
                attachments TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        println!("Created deleted_msgs table");
        Ok(())
    }

    pub async fn insert_msg_logs(
        &self,

        msg_id: i64,
        guild_id: i64,
        channel_id: i64,
        author_id: i64,
        contents: &str,
        attachments: Vec<String>,
    ) -> Result<(), Error> {
        let attachment_string = match attachments.len() {
            0 => "".to_string(), // Return an empty string if the attachments vector is empty
            _ => attachments.join(","), // Join the attachments into a single string separated by ","
        };

        let mut transaction = self.pool.begin().await?;

        let q = sqlx::query("INSERT INTO msg_logs (msg_id, guild_id, channel_id, author_id, contents, attachments) VALUES ($1, $2, $3, $4, $5, $6)");

        q.bind(msg_id)
            .bind(guild_id)
            .bind(channel_id)
            .bind(author_id)
            .bind(contents)
            .bind(attachment_string)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;

        let trans = self.pool.begin().await?;
        let row_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM msg_logs")
            .fetch_one(&self.pool)
            .await?;
        trans.commit().await?;

        if row_count > 1000 {
            let trans = self.pool.begin().await?;
            sqlx::query(
                "DELETE FROM msg_logs WHERE msg_id = (SELECT msg_id FROM logs ORDER BY msg_id ASC LIMIT 1)"
            ).execute(&self.pool).await?;
            trans.commit().await?;
        }

        Ok(())
    }

    pub async fn insert_deleted_msgs(
        &self,

        msg_id: i64,
        guild_id: i64,
        channel_id: i64,
        author_id: i64,
        contents: &str,
        attachments: Vec<String>,
    ) -> Result<(), Error> {
        let attachment_string = match attachments.len() {
            0 => "".to_string(), // Return an empty string if the attachments vector is empty
            _ => attachments.join(","), // Join the attachments into a single string separated by ","
        };

        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "INSERT INTO deleted_msgs (msg_id, guild_id, channel_id, author_id, contents, attachments) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(msg_id)
        .bind(guild_id)
        .bind(channel_id)
        .bind(author_id)
        .bind(contents)
        .bind(&attachment_string)
        .execute(&mut *transaction ).await?;
        transaction.commit().await?;

        let mut transaction = self.pool.begin().await?;
        let row_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM msg_logs")
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;
        let row_count: i64 = row_count as i64;

        if row_count > 1000 {
            let mut transaction = self.pool.begin().await?;
            sqlx::query(
            "DELETE FROM msg_logs WHERE msg_id = (SELECT msg_id FROM logs ORDER BY msg_id ASC LIMIT 1)"
        ).execute(&mut *transaction).await?;
            transaction.commit().await?;
        }

        Ok(())
    }

    pub async fn read_logs_by_id(&self, msg_id: i64) -> Result<Vec<MsgLogs>, Error> {
        let rows = sqlx::query("SELECT * FROM msg_logs WHERE msg_id = $1")
            .bind(msg_id)
            .fetch_all(&self.pool)
            .await?;

        let mut msg_logs = Vec::new();

        for row in rows {
            msg_logs.push(Self::parse_msg_logs_record(row)?);
        }

        Ok(msg_logs)
    }

    pub async fn get_last_deleted_msgs(&self, guild_id: i64) -> Result<Vec<MsgLogs>, Error> {
        let rows = sqlx::query(
            "SELECT * FROM deleted_msgs WHERE guild_id = $1 ORDER BY msg_id DESC LIMIT 1",
        )
        .bind(guild_id)
        .fetch_all(&self.pool)
        .await?;

        println!("Got rows");
        let mut msg_logs = Vec::new();

        for row in rows {
            msg_logs.push(Self::parse_msg_logs_record(row)?);
        }

        println!("Got msg logs");
        Ok(msg_logs)
    }

    pub async fn update_logs_by_id(&self, msg_id: i64, new_contents: &str) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query("UPDATE msg_logs SET content = $1 WHERE msg_id = $2")
            .bind(new_contents)
            .bind(msg_id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }

    fn parse_msg_logs_record(row: sqlx::postgres::PgRow) -> Result<MsgLogs, sqlx::Error> {
        Ok(MsgLogs {
            msg_id: row.try_get("msg_id")?,
            guild_id: row.try_get("guild_id")?,
            channel_id: row.try_get("channel_id")?,
            author_id: row.try_get("author_id")?,
            contents: row.try_get("contents")?,
            attachments: row.try_get("attachments")?,
        })
    }
}
