use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
};
use sqlx::SqlitePool;

use crate::{
    error_handle::AppError,
    notes::repository::{
        InsertNote, Note, RawInsertNote, RawUpdateNote, UpdateNote, delete_note, insert_note,
        select_notes, update_note,
    },
};

async fn select(State(pool): State<SqlitePool>) -> Result<Json<Vec<Note>>, AppError> {
    Ok(Json(select_notes(&pool).await?))
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(raw): Json<RawInsertNote>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    let input = InsertNote::try_from(raw)?;
    insert_note(input, &pool)
        .await
        .map(|note| (StatusCode::CREATED, Json(note)))
}

async fn update(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(raw): Json<RawUpdateNote>,
) -> Result<Json<Note>, AppError> {
    let input = UpdateNote::try_from(raw)?;
    Ok(Json(update_note(id, input, &pool).await?))
}

async fn delete(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Note>, AppError> {
    Ok(Json(delete_note(id, &pool).await?))
}

pub fn notes_router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(select).post(create))
        .route("/{id}", patch(update).delete(delete))
}
