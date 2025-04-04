use std::sync::Arc;

use axum::{
    Json, Router, debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa::{
    Modify,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    context::AppState,
    controllers::{auth, tasks},
    middlewares::{auth::JwtAuthLayer, trace},
};

#[derive(OpenApi)]
#[openapi(
    tags((name = "Health Check", description = "Health check handler API")),
    modifiers(&SecurityAddOn)
)]
struct ApiDoc;

struct SecurityAddOn;

impl Modify for SecurityAddOn {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

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

pub fn router(ctx: &AppState) -> Router {
    let app_router = OpenApiRouter::new()
        .with_state(Arc::new(ctx.clone()))
        .nest("/auth", auth::auth_routes(&ctx))
        .nest(
            "/tasks",
            tasks::task_routes(&ctx).layer(JwtAuthLayer::new(&ctx)),
        )
        .routes(routes!(health))
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
        .merge(Scalar::with_url("/", open_api));

    Router::new().merge(router)
}
