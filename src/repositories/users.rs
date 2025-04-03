use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{DateTime, FixedOffset, Utc};
use jsonwebtoken::{Algorithm, Header};
use serde::Deserialize;
use sqlx::{Encode, Executor, PgPool, Postgres, prelude::FromRow};
use uuid::Uuid;

use crate::{
    context::JwtState,
    models::auth::{LoginResponse, LoginUser, RegisterUser, TokenClaims},
};

use super::ModelError;

#[derive(Debug, Deserialize, Clone, FromRow, Encode)]
pub struct User {
    pub id: i32,
    pub pid: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl User {
    pub async fn create_with_password(db: &PgPool, dto: &RegisterUser) -> Result<Self, ModelError> {
        let mut txn = db.begin().await?;

        let password_hashed = Argon2::default()
            .hash_password(&dto.password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|hash| hash.to_string())?;

        let result = sqlx::query_as::<_, Self>(
            "
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3) RETURNING *
            ",
        )
        .bind(&dto.username)
        .bind(&dto.email)
        .bind(password_hashed)
        .fetch_one(&mut *txn)
        .await;

        if let Err(sqlx::Error::Database(err)) = result {
            match err.constraint() {
                Some("users_username_key") => return Err(ModelError::UsernameTaken),
                Some("users_email_key") => return Err(ModelError::EmailExists),
                _ => return Err(ModelError::Sqlx(sqlx::Error::Database(err))),
            }
        }

        let model = result?;

        txn.commit().await?;

        Ok(model)
    }

    pub async fn find_by_email<'e, C>(db: C, email: &str) -> Result<Self, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await?;

        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_by_username<'e, C>(db: C, username: &str) -> Result<Self, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(db)
            .await?;

        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_by_pid<'e, C>(db: C, pid: Uuid) -> Result<Self, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE pid = $1")
            .bind(pid)
            .fetch_optional(db)
            .await?;

        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn login_user(
        db: &PgPool,
        dto: &LoginUser,
        auth: &JwtState,
    ) -> Result<LoginResponse, ModelError> {
        let user = match Self::find_by_email(db, &dto.email).await {
            Ok(user) => user,
            Err(e) => match e {
                ModelError::EntityNotFound => return Err(ModelError::Unauthorised),
                _ => return Err(e),
            },
        };

        let password_hash = PasswordHash::new(&user.password)?;

        Argon2::default()
            .verify_password(&dto.password.as_bytes(), &password_hash)
            .map_err(|e| {
                tracing::warn!("An error occurred: {e}");
                ModelError::Unauthorised
            })?;

        let now = Utc::now();

        let claims = TokenClaims {
            sub: user.pid.to_string(),
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::seconds(auth.max_age as i64)).timestamp() as usize,
        };

        let header = Header::new(Algorithm::RS256);

        let token = jsonwebtoken::encode(&header, &claims, &auth.encoding_key)?;

        Ok(LoginResponse::new(&user, &token))
    }
}
