use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, Response},
};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{Span, field};

pub(crate) fn make_span_with(request: &Request<Body>) -> Span {
    tracing::info_span!(
        "app",
        version = field::debug(request.version()),
        uri = field::display(request.uri()),
        method = field::display(request.method()),
        status = field::Empty,
        latency = field::Empty,
        error = field::Empty
    )
}

pub(crate) fn on_request(_request: &Request<Body>, _span: &Span) {
    tracing::info!("->");
}

pub(crate) fn on_response(response: &Response<Body>, latency: Duration, span: &Span) {
    span.record("status", field::display(response.status()));
    span.record(
        "latency",
        field::display(format!("{}µs", latency.as_millis())),
    );
    tracing::info!("<-");
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn on_failure(error: ServerErrorsFailureClass, latency: Duration, span: &Span) {
    span.record("error", field::display(error.to_string()));
    span.record(
        "latency",
        field::display(format!("{}µs", latency.as_millis())),
    );
    tracing::error!("Error on request");
}
