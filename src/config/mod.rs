pub mod db;
pub mod jwt;
pub mod logger;

pub use self::{
    db::DatabaseConfig,
    jwt::{AuthConfig, RsaJwtConfig},
    logger::Telemetry,
};

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize, Default, Clone)]
pub enum AppEnvironment {
    #[default]
    Development,
    Production,
    Testing,
    Other(String),
}

impl std::fmt::Display for AppEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env = match self {
            Self::Development => "development",
            Self::Production => "production",
            Self::Testing => "testing",
            Self::Other(env) => env,
        };

        write!(f, "{env}")
    }
}

impl From<String> for AppEnvironment {
    fn from(env: String) -> Self {
        match env.trim().to_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "production" | "prod" => Self::Production,
            "testing" | "test" => Self::Testing,
            _ => Self::Other(env),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub(crate) protocol: String,
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl std::fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}", &self.protocol, &self.host, self.port)
    }
}

impl ServerConfig {
    #[must_use]
    pub fn address(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub(crate) server: ServerConfig,
    pub(crate) logger: Telemetry,
    pub(crate) db: DatabaseConfig,
    pub(crate) auth: AuthConfig,
}

impl AppConfig {
    /// Function that builds the [`AppConfig`] from a file.
    ///
    /// # Errors
    /// * IO Errors
    /// * File system errors
    /// * Deserialising errors
    pub fn from_env(env: &AppEnvironment) -> Result<Self, Error> {
        // The config file must live in config/ directory and must be yaml.
        let base_dir = std::env::current_dir()?;
        let config_dir = base_dir.join("config");
        let config_file_path = config_dir.join(format!("{env}.yaml"));

        let settings = config::Config::builder()
            .add_source(config::File::from(config_file_path))
            .build()?;

        settings.try_deserialize::<Self>().map_err(Into::into)
    }
}

impl AppConfig {
    #[must_use]
    pub const fn server(&self) -> &ServerConfig {
        &self.server
    }

    #[must_use]
    pub const fn logger(&self) -> &Telemetry {
        &self.logger
    }

    #[must_use]
    pub const fn db(&self) -> &DatabaseConfig {
        &self.db
    }

    #[must_use]
    pub const fn auth(&self) -> &AuthConfig {
        &self.auth
    }
}
