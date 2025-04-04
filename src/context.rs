use std::path::PathBuf;

use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

use crate::{AppConfig, Error};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
    pub jwt: JwtState,
}

impl AppState {
    pub fn new(config: &AppConfig) -> Result<Self, Error> {
        let db = config.db.connection_pool()?;
        let jwt = JwtState::new(
            &config.auth.access.private_key,
            &config.auth.access.public_key,
            config.auth.access.expiration,
        )?;
        Ok(Self {
            config: config.clone(),
            db,
            jwt,
        })
    }
}

#[derive(Clone)]
pub struct JwtState {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub max_age: u64,
}

impl JwtState {
    pub fn new(
        private_key_path: &PathBuf,
        public_key_path: &PathBuf,
        max_age: u64,
    ) -> Result<Self, Error> {
        let private_key = std::fs::read_to_string(private_key_path)?;
        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

        let public_key = std::fs::read_to_string(public_key_path)?;
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;

        Ok(Self {
            encoding_key,
            decoding_key,
            max_age,
        })
    }
}
