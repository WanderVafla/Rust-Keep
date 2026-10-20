use axum::{Router, routing::get};

mod db;
mod error_handle;
mod notes;
use notes::router::notes_router;

use crate::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not present")?;
    let pool = create_pool(&database_url).await?;
    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/notes", notes_router())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
