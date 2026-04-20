use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn setup_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    tracing::info!("Running database migrations...");
    sqlx::migrate!("../migrations").run(&pool).await?;
    tracing::info!("Migrations applied successfully!");

    Ok(pool)
}
