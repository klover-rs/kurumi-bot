use crate::secrets::get_secret;
use crate::Error;
use sqlx::PgPool;
use sqlx::Row;

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

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS muted (
        uid BIGINT PRIMARY KEY,
        guild_id BIGINT,
        reason TEXT,
        roles TEXT,
        duration BIGINT
    )"#,
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

        let q = sqlx::query(
            "INSERT INTO muted (uid, guild_id, reason, roles, duration) VALUES ($1, $2, $3, $4, $5)",
          
        );

        q.bind(uid)
            .bind(guild_id)
            .bind(reason)
            .bind(roles_str)
            .bind(duration)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    pub async fn read_muted(&self) -> Result<Vec<Muted>, Error> {
        let mut muted_records = Vec::new();

        let rows = sqlx::query("SELECT * FROM muted")
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            muted_records.push(parse_muted_record(row)?);
        }

        Ok(muted_records)
    }

    pub async fn read_muted_by_uid(&self, uid: i64) -> Result<Vec<Muted>, Error> {
        let mut muted_records = Vec::new();
        let rows = sqlx::query("SELECT * FROM muted WHERE uid = $1")
            .bind(uid)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            muted_records.push(parse_muted_record(row)?);
        }

        Ok(muted_records)
    }

    pub async fn delete(&self, uid: i64) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;

        let q = sqlx::query("DELETE FROM muted WHERE uid = $1");

        q.bind(uid).execute(&mut *transaction).await?;

        transaction.commit().await?;
        Ok(())
    }
}

fn parse_muted_record(row: sqlx::postgres::PgRow) -> Result<Muted, Error> {
    Ok(Muted {
        uid: row.try_get(0)?,
        guild_id: row.try_get(1)?,
        reason: row.try_get(2)?,
        roles: row.try_get(3)?,
        duration: row.try_get(4)?,
    })
}
