use crate::{AppConfig, AppEnvironment, Error};

use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    about = "Web service that handles task tracking",
    author = "Simon Bittok <bittokks@gmail.com>"
)]
pub struct App {
    #[arg(long, short, default_value_t = AppEnvironment::default())]
    env: AppEnvironment,
}

impl App {
    /// Function that runs the web service asynchronously to completion
    ///
    /// # Errors
    /// * Any variant of the Error enum in the errors module.
    pub async fn run() -> Result<(), Error> {
        let cli = Self::parse();
        let config = AppConfig::from_env(&cli.env)?;

        let logger = config.logger();
        logger.setup()?;

        let listener = TcpListener::bind(config.server.address()).await?;
        let router = crate::router::router();

        println!("Running on: {}", config.server());
        axum::serve(listener, router).await.map_err(Into::into)
    }
}
