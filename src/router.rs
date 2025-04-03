use std::sync::Arc;

use axum::{
    Json, Router, debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{context::AppState, middlewares::trace};

#[derive(OpenApi)]
#[openapi(
    tags((name = "Health Check", description = "Health check handler API"))
)]
struct ApiDoc;

#[derive(Debug, serde::Serialize, Clone, utoipa::ToSchema)]
pub struct GenericResponse {
    pub message: String,
}

#[debug_handler]
#[utoipa::path(get, path = "/health", responses((status = OK, body = GenericResponse )), tag = "Health" )]
async fn health() -> Response {
    let message = GenericResponse {
        message: "Server is up and running".into(),
    };

    (StatusCode::OK, Json(message)).into_response()
}

#[debug_handler]
#[utoipa::path(get, path = "/error", responses((status = INTERNAL_SERVER_ERROR, body = GenericResponse)), tag = "Error" )]
async fn error() -> Result<Response, impl IntoResponse> {
    let _message = json!({
        "message": "This endpoint should return an error"
    });

    Err(crate::Error::IO(
        std::io::ErrorKind::PermissionDenied.into(),
    ))
}

pub fn router(ctx: AppState) -> Router {
    let app_router = OpenApiRouter::new()
        .routes(routes!(health))
        .routes(routes!(error))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::make_span_with)
                .on_request(trace::on_request)
                .on_response(trace::on_response)
                .on_failure(trace::on_failure),
        );

    let (router, open_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", app_router)
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", open_api.clone()))
        .merge(Redoc::with_url("/redoc", open_api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/", open_api))
        .with_state(Arc::new(ctx));

    Router::new().merge(router)
}
