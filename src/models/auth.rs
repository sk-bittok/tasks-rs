use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repositories::users::User;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Validate)]
pub struct RegisterUser {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(
        min = 5,
        max = 50,
        message = "Username must be between 5 to 50 characters long"
    ))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 48,
        message = "Password must be between 8 to 48 characters long"
    ))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Validate)]
pub struct LoginUser {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
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

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
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
    pub exp: usize,
}
