use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::error::AppError;

/// The caller's identity. For now it is read from the `X-User-Id` header
/// (a UUID). Swap this extractor for JWT/session validation later and no
/// handler or repository code needs to change.
pub struct CurrentUser(pub Uuid);

impl<S: Send + Sync> FromRequestParts<S> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, AppError> {
        parts
            .headers
            .get("x-user-id")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .map(CurrentUser)
            .ok_or(AppError::Unauthorized)
    }
}
