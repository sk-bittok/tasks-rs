use serde::{Deserialize, Serialize};

use crate::repositories::users::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub message: String,
}

impl AuthResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub created_at: String,
}

impl LoginResponse {
    pub fn new(user: &User, token: &str) -> Self {
        Self {
            token: token.into(),
            username: user.username.to_string(),
            created_at: user.created_at.format("%d-%m-%Y %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

