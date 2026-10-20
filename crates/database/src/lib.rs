pub mod models;

pub type PgPool = sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    sqlx::PgPool::connect(database_url).await
}
