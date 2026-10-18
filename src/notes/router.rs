use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
};
use sqlx::SqlitePool;

use crate::notes::repository::{
    Note, UpsterNote, delete_note, insert_note, select_notes, update_note,
};

fn db_error(e: sqlx::Error) -> (StatusCode, String) {
    match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Note not found".to_string()),
        e => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn select(State(pool): State<SqlitePool>) -> Result<Json<Vec<Note>>, (StatusCode, String)> {
    select_notes(&pool).await.map_err(db_error)
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(input): Json<UpsterNote>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    insert_note(input, &pool)
        .await
        .map(|note| (StatusCode::CREATED, note))
        .map_err(db_error)
}

async fn update(
    State(pool): State<SqlitePool>,
    Path(id): Path<u32>,
    Json(input): Json<UpsterNote>,
) -> Result<Json<Note>, (StatusCode, String)> {
    update_note(id, input, &pool).await.map_err(db_error)
}

async fn delete(
    Path(id): Path<u32>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Note>, (StatusCode, String)> {
    delete_note(id, &pool).await.map_err(db_error)
}

pub fn notes_router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(select).post(create))
        .route("/{id}", patch(update).delete(delete))
}
