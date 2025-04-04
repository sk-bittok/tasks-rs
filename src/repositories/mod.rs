pub mod tasks;
pub mod users;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{0}")]
    Argon2(argon2::Error),
    #[error("{0}")]
    ArgonPasswordHash(argon2::password_hash::Error),
    #[error("Account with email already exists")]
    EmailExists,
    #[error("Entity not in the database")]
    EntityNotFound,
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Failed to authenticate user")]
    Unauthorised,
    #[error("Username already taken")]
    UsernameTaken,
}

impl ModelError {
    pub fn response(&self) -> Response {
        let (status, message) = match self {
            Self::EmailExists => (
                StatusCode::CONFLICT,
                "Email already registered to an account",
            ),
            Self::EntityNotFound => (StatusCode::NOT_FOUND, "Entity not found"),
            Self::Sqlx(_) | Self::Argon2(_) | Self::ArgonPasswordHash(_) | Self::Jwt(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end",
            ),
            Self::UsernameTaken => (
                StatusCode::CONFLICT,
                "Username is already taken, please pick another one",
            ),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED, "Invalid email or password"),
        };

        let body = Json(json!({
            "message": message
        }));

        (status, body).into_response()
    }
}

impl From<argon2::Error> for ModelError {
    fn from(value: argon2::Error) -> Self {
        Self::Argon2(value)
    }
}

impl From<argon2::password_hash::Error> for ModelError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::ArgonPasswordHash(value)
    }
}
