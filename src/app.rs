use std::io::IsTerminal;

use crate::{AppConfig, AppEnvironment, Error, context::AppState};

use clap::Parser;
use dotenv::dotenv;
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
        dotenv().ok();

        let cli: Self = Self::parse();
        let config: AppConfig = AppConfig::from_env(&cli.env)?;

        color_eyre::config::HookBuilder::default()
            .theme(if std::io::stdout().is_terminal() {
                color_eyre::config::Theme::dark()
            } else {
                color_eyre::config::Theme::new()
            })
            .install()?;

        let logger = config.logger();
        logger.setup()?;

        config.db().migrate().await?;

        let listener: TcpListener = TcpListener::bind(config.server.address()).await?;
        let router = crate::router::router(&AppState::new(&config)?);

        println!("Running on: {}", config.server());
        axum::serve(listener, router).await.map_err(Into::into)
    }
}
