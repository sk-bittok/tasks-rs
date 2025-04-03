pub mod response;

use tracing_subscriber::{filter::FromEnvError, util::TryInitError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error("{0}")]
    ColorEyre(#[from] color_eyre::Report),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Env(#[from] FromEnvError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Parse(#[from] tracing_subscriber::filter::ParseError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    TryInit(#[from] TryInitError),
}
