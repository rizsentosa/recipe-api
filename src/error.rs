use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Validation(String),
    #[error("resource not found")]
    NotFound,
    #[error("missing or invalid X-User-Id header")]
    Unauthorized,
    #[error("internal server error")]
    Internal,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db) if db.is_foreign_key_violation() => AppError::NotFound,
            _ => {
                tracing::error!("database error: {e}");
                AppError::Internal
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
