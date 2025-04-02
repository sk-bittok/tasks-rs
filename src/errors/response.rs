use super::Error;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl Error {
    pub fn response(&self) -> Response {
        tracing::error!("An error occurred: {:?}", &self);
        let (status, message) = match self {
            Self::Axum(_)
            | Self::Config(_)
            | Self::Env(_)
            | Self::IO(_)
            | Self::Parse(_)
            | Self::TryInit(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end.",
            ),
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
