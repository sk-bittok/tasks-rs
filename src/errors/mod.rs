pub mod response;

use tracing_subscriber::{filter::FromEnvError, util::TryInitError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Env(#[from] FromEnvError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] tracing_subscriber::filter::ParseError),
    #[error(transparent)]
    TryInit(#[from] TryInitError),
}
