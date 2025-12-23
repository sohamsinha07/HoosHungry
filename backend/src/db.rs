use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn connect(db_url: &str) -> anyhow::Result<PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;
    Ok(pool)
}
