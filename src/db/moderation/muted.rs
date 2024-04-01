use crate::secrets::get_secret;
use crate::Error;

use sqlx::PgPool;

pub struct Database {
    pub pool: PgPool,
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
        // Note the change in the return type here

        sqlx::query!(
            r#"CREATE TABLE IF NOT EXISTS muted (
        uid BIGINT PRIMARY KEY,
        guild_id BIGINT,
        reason TEXT,
        roles TEXT,
        duration BIGINT
    )"#
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert(
        &self,

        uid: i64,
        guild_id: i64,
        reason: &str,
        roles: Vec<u64>,
        duration: i64,
    ) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        let roles_str = roles
            .iter()
            .map(|&role| role.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let trans = self.pool.begin().await?;
        let q = sqlx::query!(
            "INSERT INTO muted (uid, guild_id, reason, roles, duration) VALUES ($1, $2, $3, $4, $5)",
            &uid, &guild_id, &reason, &roles_str, &duration
        );

        q.execute(&mut *transaction).await?;
        trans.commit().await?;
        Ok(())
    }

    pub async fn read_muted(&self) -> Result<Vec<Muted>, Error> {
        let mut muted_records = Vec::new();

        let rows = sqlx::query!("SELECT * FROM muted")
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            muted_records.push(Muted {
                uid: row.uid,
                guild_id: row.guild_id.unwrap_or(0),
                reason: row.reason.unwrap_or("".to_string()),
                roles: row.roles.unwrap_or("".to_string()),
                duration: row.duration.unwrap_or(0),
            });
        }

        Ok(muted_records)
    }

    pub async fn read_muted_by_uid(&self, uid: i64) -> Result<Vec<Muted>, Error> {
        let mut muted_records = Vec::new();
        let rows = sqlx::query!("SELECT * FROM muted WHERE uid = $1", &uid)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            muted_records.push(Muted {
                uid: row.uid,
                guild_id: row.guild_id.unwrap_or(0),
                reason: row.reason.unwrap_or("".to_string()),
                roles: row.roles.unwrap_or("".to_string()),
                duration: row.duration.unwrap_or(0),
            });
        }

        Ok(muted_records)
    }

    pub async fn delete(&self, uid: i64) -> Result<(), Error> {
        let trans = self.pool.begin().await?;
        let mut transaction = self.pool.begin().await?;

        let q = sqlx::query!("DELETE FROM muted WHERE uid = $1", &uid);

        q.execute(&mut *transaction).await?;

        trans.commit().await?;
        Ok(())
    }
}
