use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::error::ErrorKind;

/* Sqlite Errors Code */
const SQLITE_BUSY: i32 = 5;
const SQLITE_LOCKED: i32 = 6;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("note not found")]
    NotFound,
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Conflict(String),
    #[error("Database unavailable.")]
    DbUnavailable(#[source] sqlx::Error),
    #[error(transparent)]
    Db(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),
            AppError::DbUnavailable(e) => {
                tracing::error!(error = %e, "db unavailable.");
                (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable.").into_response()
            }

            AppError::Db(e) => {
                tracing::error!(error = %e, "db error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => AppError::DbUnavailable(e),
            sqlx::Error::Database(db) => {
                // extended result code → primary code (517 SQLITE_BUSY_SNAPSHOT → 5 SQLITE_BUSY)
                let base = db
                    .code() // db.code = Some("123") string
                    .and_then(|c| c.parse::<i32>().ok()) // "517" → Some(517); parse error → None
                    .map(|c| c & 0xff); // 0xff = 255, (code AND 255)

                if matches!(base, Some(SQLITE_BUSY | SQLITE_LOCKED)) {
                    return AppError::DbUnavailable(e);
                }
                match db.kind() {
                    ErrorKind::UniqueViolation => AppError::Conflict("already exists".into()),
                    ErrorKind::NotNullViolation => {
                        tracing::error!(error = %db, "constraint violation"); // With current db table "NotNullViolation" is impossible
                        AppError::Validation("required field is missing".into())
                    }
                    ErrorKind::CheckViolation if db.message().ends_with("note_not_empty") => {
                        AppError::Validation("note is empty".into())
                    }
                    ErrorKind::CheckViolation => {
                        AppError::Validation("value is not allowed".into())
                    }
                    ErrorKind::ForeignKeyViolation => {
                        AppError::Validation("related record does not exist".into())
                    }
                    _ => AppError::Db(e),
                }
            }
            _ => AppError::Db(e),
        }
    }
}
