use std::sync::Arc;

use axum::{
    Extension, Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState, Result,
    errors::response::ErrorResponse,
    middlewares::auth::AuthClaims,
    models::{
        Validator,
        auth::TokenClaims,
        tasks::{NewTask, TaskResponse, UpdateTask},
    },
    repositories::tasks::Task,
};

const TASK_TAG: &str = "Tasks";

/// Create new task
///
/// Attempts to create a new [`Task`] in the database
#[debug_handler]
#[utoipa::path(
    tag = TASK_TAG,
    post,
    path = "/",
    security(("token" = [])),
    request_body(content = NewTask, content_type = "application/json", description = "Data to create a new task"),
    responses(
        (status = 201, body = TaskResponse, description = "Successful task creation"),
        (status = 401, body = ErrorResponse, description = "Invalid authorisation token"),
        (status = 500, body = ErrorResponse, description = "Internal server errors")
    )
)]
async fn add(
    State(ctx): State<Arc<AppState>>,
    Extension(auth): Extension<TokenClaims>,
    Json(params): Json<NewTask>,
) -> Result<Response> {
    let validator = Validator::new(params);
    let dto = validator.validate()?;

    let task = Task::create_task(&ctx.db, dto, Uuid::parse_str(&auth.sub).unwrap()).await?;

    Ok((StatusCode::CREATED, Json(TaskResponse::from(task))).into_response())
}

/// Get list of tasks
///
/// Attempts to get a list of [`Task`] from the database
#[debug_handler]
#[utoipa::path(
    tag = TASK_TAG,
    get,
    path = "/",
    security(("token" = [])),
    responses(
        (status = 200, body = Vec<TaskResponse>, description = "Successful tasks retrieval"),
        (status = 401, body = ErrorResponse, description = "Invalid authorisation token"),
        (status = 500, body = ErrorResponse, description = "Internal server errors")
    )
)]
async fn all(
    State(ctx): State<Arc<AppState>>,
    Extension(auth): Extension<TokenClaims>,
) -> Result<Response> {
    let tasks = Task::find_all(&ctx.db, Uuid::parse_str(&auth.sub).unwrap())
        .await?
        .into_iter()
        .map(TaskResponse::from)
        .collect::<Vec<TaskResponse>>();

    Ok((StatusCode::OK, Json(tasks)).into_response())
}

/// Get task by its ID
///
/// Attempts to get a [`Task`] by its ID from the database
#[debug_handler]
#[utoipa::path(
    tag = TASK_TAG,
    get,
    path = "/{id}",
    params(("id" = i32, Path, description = "Task ID")),
    security(("token" = [])),
    responses(
        (status = 200, body = TaskResponse, description = "Successful task retrieval"),
        (status = 401, body = ErrorResponse, description = "Invalid authorisation token"),
        (status = 500, body = ErrorResponse, description = "Internal server errors")
    )
)]
async fn one(
    State(ctx): State<Arc<AppState>>,
    Extension(auth): Extension<AuthClaims>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let task = Task::find_by_id(&ctx.db, auth.pid(), id).await?;

    Ok((StatusCode::OK, Json(TaskResponse::from(task))).into_response())
}

/// Delete a task
///
/// Attempts to delete  a [`Task`] by its ID from the database
/// Only creator(`User`) of the task can delete it.
#[debug_handler]
#[utoipa::path(
    tag = TASK_TAG,
    delete,
    path = "/{id}",
    params(("id" = i32, Path, description = "Task ID")),
    security(("token" = [])),
    responses(
        (status = 204, description = "Successful task deletion"),
        (status = 401, body = ErrorResponse, description = "Invalid authorisation token"),
        (status = 500, body = ErrorResponse, description = "Internal server errors")
    )
)]
async fn remove(
    State(ctx): State<Arc<AppState>>,
    Extension(auth): Extension<AuthClaims>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let query = Task::delete_by_id(&ctx.db, id, auth.pid()).await?;
    tracing::info!("Deleted rows {}", query.rows_affected());
    Ok((StatusCode::NO_CONTENT, Json({})).into_response())
}

/// Update a task
///
/// Attempts to update  a [`Task`] by its ID inside the database
/// Only creator (`User`) of the task can update it.
#[debug_handler]
#[utoipa::path(
    tag = TASK_TAG,
    patch,
    path = "/{id}",
    params(("id" = i32, Path, description = "Task ID")),
    security(("token" = [])),
    responses(
        (status = 201, body= TaskResponse , description = "Successful task update"),
        (status = 401, body = ErrorResponse, description = "Invalid authorisation token"),
        (status = 500, body = ErrorResponse, description = "Internal server errors")
    )
)]
async fn update(
    State(ctx): State<Arc<AppState>>,
    Extension(auth): Extension<AuthClaims>,
    Path(id): Path<i32>,
    Json(params): Json<UpdateTask>,
) -> Result<Response> {
    let task = Task::update_by_id(&ctx.db, &params, id, auth.pid()).await?;

    Ok((StatusCode::CREATED, Json(TaskResponse::from(task))).into_response())
}

pub fn task_routes(ctx: &AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(add))
        .routes(routes!(all))
        .routes(routes!(one))
        .routes(routes!(remove))
        .routes(routes!(update))
        .with_state(Arc::new(ctx.clone()))
}
