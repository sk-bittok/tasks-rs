use std::sync::Arc;

use axum::{
    Extension, Json, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState, Result,
    errors::response::ErrorResponse,
    models::{
        Validator,
        auth::TokenClaims,
        tasks::{NewTask, TaskResponse},
    },
    repositories::tasks::Task,
};

const TASK_TAG: &str = "Tasks";

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

pub fn task_routes(ctx: &AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(add))
        .with_state(Arc::new(ctx.clone()))
}
