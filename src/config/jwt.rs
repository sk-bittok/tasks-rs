use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RsaJwtConfig {
    pub private_key: PathBuf,
    pub public_key: PathBuf,
    pub expiration: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub access: RsaJwtConfig,
}
