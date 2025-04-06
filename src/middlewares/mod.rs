pub mod auth;
pub mod trace;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    ExpiredToken(String),
    #[error("{0}")]
    ImmatureToken(String),
    #[error("Token is invalid")]
    InvalidToken,
    #[error("{0:?}")]
    JsonWebToken(jsonwebtoken::errors::ErrorKind),
    #[error("Credentials not provided in the request")]
    MissingCredentials,
    #[error("Provided credentials is wrong")]
    WrongCredentials,
    #[error("{0}")]
    Other(String),
}

impl AuthError {
    pub fn response(&self) -> Response {
        let (status, message) = match self {
            Self::ExpiredToken(e) => (StatusCode::UNAUTHORIZED, e.as_str()),
            Self::ImmatureToken(e) => (StatusCode::FORBIDDEN, e.as_str()),
            Self::InvalidToken => (StatusCode::FORBIDDEN, "Invalid authorisation token"),
            Self::JsonWebToken(e) => {
                tracing::error!("JsonWebToken Error {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong on our end.",
                )
            }
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Self::Other(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end",
            ),
        };

        let body = Json(serde_json::json!({
            "message": message
        }));

        (status, body).into_response()
    }
}
