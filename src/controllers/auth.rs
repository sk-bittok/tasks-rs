use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::json;
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
        (status=200, description="User logged-in succesfully", body=LoginResponse, content_type = "application/json"),
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

    let access_cookie = Cookie::build(("accessToken", &user.token))
        .path("/")
        .max_age(time::Duration::seconds(ctx.jwt.max_age as i64))
        .same_site(SameSite::Lax)
        .http_only(true);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Authorization", format!("Bearer {}", &user.token))
        .header(SET_COOKIE, access_cookie.to_string())
        .header("Content-Type", "application/json")
        .body(Body::new(json!(user).to_string()))?;

    Ok(response)
}

pub fn auth_routes(ctx: &AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login))
        .with_state(Arc::new(ctx.clone()))
}
