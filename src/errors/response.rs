use super::Error;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

impl Error {
    pub fn response(&self) -> Response {
        tracing::error!("An error occurred: {:?}", &self);
        let (status, message) = match self {
            Self::Axum(_)
            | Self::AxumHttp(_)
            | Self::ColorEyre(_)
            | Self::Config(_)
            | Self::Env(_)
            | Self::IO(_)
            | Self::JsonWebToken(_)
            | Self::Parse(_)
            | Self::Sqlx(_)
            | Self::SqlxMigrate(_)
            | Self::TryInit(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end.",
            ),
            Self::Validation(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.as_str()),
            Self::Model(e) => return e.response(),
        };

        let body = serde_json::json!({
            "message": message
        });

        (status, Json(body)).into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}
