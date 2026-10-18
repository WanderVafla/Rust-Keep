use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let opts = SqliteConnectOptions::new()
        .filename("database.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts.clone()).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
