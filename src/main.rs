use axum::{Router, routing::get};

mod db;
mod notes;
use notes::router::notes_router;

use crate::db::create_pool;

#[tokio::main]
async fn main() {
    let pool = create_pool().await.expect("Database Error");
    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/notes", notes_router())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("listener Error");
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect_err("axum serve Error");
}
