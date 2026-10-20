use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error_handle::AppError;

#[derive(Serialize)]
pub struct Note {
    id: i64,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
pub struct RawInsertNote {
    #[serde(default)]
    title: String,
    #[serde(default)]
    body: String,
}

pub struct InsertNote {
    title: String,
    body: String,
}

impl TryFrom<RawInsertNote> for InsertNote {
    type Error = AppError;
    fn try_from(raw: RawInsertNote) -> Result<Self, Self::Error> {
        if raw.title.trim().is_empty() && raw.body.trim().is_empty() {
            return Err(AppError::Validation("note is empty".into()));
        }
        Ok(InsertNote {
            title: raw.title,
            body: raw.body,
        })
    }
}

#[derive(Deserialize)]
pub struct RawUpdateNote {
    title: Option<String>,
    body: Option<String>,
}

pub struct UpdateNote {
    title: Option<String>,
    body: Option<String>,
}

impl TryFrom<RawUpdateNote> for UpdateNote {
    type Error = AppError;
    fn try_from(raw: RawUpdateNote) -> Result<Self, Self::Error> {
        if raw.title.is_none() && raw.body.is_none() {
            return Err(AppError::Validation("nothing to update".to_string()));
        }
        if raw.title.as_deref().is_some_and(|s| s.trim().is_empty())
            && raw.body.as_deref().is_some_and(|s| s.trim().is_empty())
        {
            return Err(AppError::Validation("note is empty".to_string()));
        }
        Ok(UpdateNote {
            title: raw.title,
            body: raw.body,
        })
    }
}

pub async fn select_notes(pool: &SqlitePool) -> Result<Vec<Note>, AppError> {
    let query = sqlx::query_as!(
        Note,
        "select * from notes
        order by
            updated_at desc, created_at desc, id desc
        ;"
    )
    .fetch_all(pool)
    .await?;
    Ok(query)
}

pub async fn insert_note(data: InsertNote, pool: &SqlitePool) -> Result<Note, AppError> {
    Ok(sqlx::query_as!(
        Note,
        "insert into notes (title, body)
        values ($1, $2)
        returning
            id,
            title,
            body,
            updated_at,
            created_at
        ;",
        data.title,
        data.body
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update_note(id: i64, data: UpdateNote, pool: &SqlitePool) -> Result<Note, AppError> {
    sqlx::query_as!(
        Note,
        "update notes
        set
            title = coalesce($2, title),
            body = coalesce($3, body),
            updated_at = datetime('now')
        where
            id = $1
        returning
            id,
            title,
            body,
            updated_at,
            created_at
        ;",
        id,
        data.title,
        data.body
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)
}

pub async fn delete_note(id: i64, pool: &SqlitePool) -> Result<Note, AppError> {
    sqlx::query_as!(
        Note,
        "delete from notes
        where
            id = $1
        returning
            id,
            title,
            body,
            updated_at,
            created_at
        ;",
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)
}
