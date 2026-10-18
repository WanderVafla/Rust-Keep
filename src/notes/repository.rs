use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Note {
    id: u32,
    title: Option<String>,
    body: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsterNote {
    title: Option<String>,
    body: String,
}

pub async fn select_notes(pool: &SqlitePool) -> Result<Json<Vec<Note>>, sqlx::Error> {

    let query: Vec<Note> = sqlx::query_as::<_, Note>("select * from notes")
        .fetch_all(pool)
        .await?;
    println!("{:?}", query);
    Ok(Json(query))
}

pub async fn insert_note(data: UpsterNote, pool: &SqlitePool) -> Result<Json<Note>, sqlx::Error> {

    let query: Note = sqlx::query_as::<_, Note>(
        "insert into notes (title, body)
        values ($1, $2)
        returning *",
    )
    .bind(data.title)
    .bind(data.body)
    .fetch_one(pool)
    .await?;

    println!("{:?}", query);
    Ok(Json(query))
}

pub async fn update_note(id: u32, data: UpsterNote, pool: &SqlitePool) -> Result<Json<Note>, sqlx::Error> {

    let query: Note = sqlx::query_as::<_, Note>(
        "update notes
        set
            title = $2,
            body = $3,
            updated_at = datetime('now')
        where
            id = $1
        returning *",
    )
    .bind(id)
    .bind(data.title)
    .bind(data.body)
    .fetch_one(pool)
    .await?;

    println!("{:?}", query);
    Ok(Json(query))
}

pub async fn delete_note(id: u32, pool: &SqlitePool) -> Result<Json<Note>, sqlx::Error> {

    let query: Note = sqlx::query_as::<_, Note>(
        "delete from notes
        where
            id = $1
        returning *",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    println!("{:?}", query);
    Ok(Json(query))
}
