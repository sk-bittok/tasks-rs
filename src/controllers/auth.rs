use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState, Result,
    errors::response::ErrorResponse,
    models::{
        Validator,
        auth::{AuthResponse, LoginResponse, LoginUser, RegisterUser},
    },
    repositories::users::User,
};

const AUTH_TAG: &str = "Auth";

/// Register a new user
///
/// # Errors
/// * Request body validation failure.
/// * User with email already exists
/// * Username is aleady taken
/// * Internal server error.
#[utoipa::path(
    tag = AUTH_TAG,
    post,
    path = "/register",
    request_body(content=RegisterUser, content_type="application/json", description="Registration data"),
    responses(
        (status=201, description="User registration success", body=AuthResponse),
        (status=422, description="Validation error on request body", body=ErrorResponse),
        (status=409, description="Username or email is already registered", body=ErrorResponse),
        (status=500, description="Internal server error", body=ErrorResponse)
    )
)]
async fn register(
    State(ctx): State<Arc<AppState>>,
    Json(params): Json<RegisterUser>,
) -> Result<Response> {
    let validator = Validator::new(params);
    let dto = validator.validate()?;

    let user = User::create_with_password(&ctx.db, dto).await?;

    tracing::info!("User {} registered successful.", &user.username);

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse::new(
            "Account created successful. Verify email",
        )),
    )
        .into_response())
}

/// Logs in a user
///
/// # Errors
/// * Request body validation failure.
/// * User or password fails to match.
/// * Internal server error.
#[utoipa::path(
    tag = AUTH_TAG,
    post,
    path = "/login",
    request_body(content=LoginUser, content_type="application/json", description="Login data"),
    responses(
        (status=200, description="User logged-in succesfully", body=LoginResponse),
        (status=422, description="Validation error on request body", body=ErrorResponse),
        (status=401, description="Email and password did not match", body=ErrorResponse),
        (status=500, description="Internal server error", body=ErrorResponse)
    )
)]
async fn login(
    State(ctx): State<Arc<AppState>>,
    Json(params): Json<LoginUser>,
) -> Result<Response> {
    let validator = Validator::new(params);
    let dto = validator.validate()?;

    let user = User::login_user(&ctx.db, dto, &ctx.jwt).await?;

    Ok((StatusCode::OK, Json(user)).into_response())
}

pub fn auth_routes(ctx: &AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login))
        .with_state(Arc::new(ctx.clone()))
}
