pub mod auth;
pub mod trace;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Token is invalid")]
    InvalidToken,
    #[error("Credentials not provided in the request")]
    MissingCredentials,
    #[error("Provided credentials is wrong")]
    WrongCredentials,
}

impl AuthError {
    pub fn response(&self) -> Response {
        let (status, message) = match self {
            Self::InvalidToken => (StatusCode::FORBIDDEN, "Invalid authorisation token"),
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
        };

        let body = Json(serde_json::json!({
            "message": message
        }));

        (status, body).into_response()
    }
}
