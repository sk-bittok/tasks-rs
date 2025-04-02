#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_self)]

use std::{error::Error, io::IsTerminal, str::FromStr};

use serde::{Deserialize, Serialize};
use tracing::Subscriber;
use tracing_subscriber::{
    EnvFilter,
    filter::Directive,
    layer::{Layer, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogFormat {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "pretty")]
    Pretty,
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::Compact => "compact",
            Self::Full => "full",
            Self::Json => "json",
            Self::Pretty => "pretty",
        };

        write!(f, "{fmt}")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "warn")]
    Warn,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self {
            Self::Debug => "debug",
            Self::Error => "error",
            Self::Info => "info",
            Self::Off => "off",
            Self::Trace => "trace",
            Self::Warn => "warn",
        };

        write!(f, "{level}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Telemetry {
    pub(crate) level: LogLevel,
    pub(crate) format: LogFormat,
    pub(crate) directives: Vec<String>,
}

impl Telemetry {
    pub fn setup(&self) -> Result<(), crate::Error> {
        let filter_layer = self.env_filter_layer()?;
        let registry = tracing_subscriber::registry()
            .with(filter_layer)
            .with(tracing_error::ErrorLayer::default());

        match &self.format {
            LogFormat::Compact => registry.with(self.compact_layer()).try_init()?,
            LogFormat::Full => registry.with(self.base_layer()).try_init()?,
            LogFormat::Json => registry.with(self.json_layer()).try_init()?,
            LogFormat::Pretty => registry.with(self.pretty_layer()).try_init()?,
        };

        Ok(())
    }

    pub fn env_filter_layer(&self) -> Result<EnvFilter, crate::Error> {
        let mut filter_layer = match EnvFilter::try_from_default_env() {
            Ok(env_filter) => env_filter,
            Err(e) => {
                if let Some(err) = e.source() {
                    match err.downcast_ref::<std::env::VarError>() {
                        Some(std::env::VarError::NotPresent) => (),
                        _ => return Err(crate::Error::Env(e)),
                    }
                }
                if self.directives.is_empty() {
                    EnvFilter::try_new(format!(
                        "{}={}",
                        env!("CARGO_PKG_NAME").replace('-', "_"),
                        &self.level
                    ))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        let parsed_directives = self.directives()?;

        for parsed_directive in parsed_directives {
            filter_layer = filter_layer.add_directive(parsed_directive);
        }

        Ok(filter_layer)
    }

    fn directives(&self) -> Result<Vec<Directive>, crate::Error> {
        self.directives
            .iter()
            .map(|directive| -> Result<Directive, crate::Error> {
                let directive_str = format!("{}={}", directive, self.level);
                Ok(Directive::from_str(&directive_str)?)
            })
            .collect()
    }

    fn is_stdout_terminal(&self) -> bool {
        std::io::stdout().is_terminal()
    }

    fn base_layer<S>(&self) -> tracing_subscriber::fmt::Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(self.is_stdout_terminal())
            .with_writer(std::io::stdout)
    }

    fn compact_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        self.base_layer()
            .compact()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
    }

    fn json_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        self.base_layer().json()
    }

    fn pretty_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        self.base_layer().pretty()
    }
}
