use crate::prelude::*;
use aide::axum::ApiRouter;
use axum::response::IntoResponse;
use axum::routing::IntoMakeService;
use axum::{
    extract::OriginalUri,
    http::{Method, StatusCode},
};
use axum::{Extension, Json, Router};
use serde_json::Value;

use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};

use super::openapi::generate;

pub fn routes(settings: Arc<Settings>) -> IntoMakeService<Router> {
    let mut openapi = generate();

    ApiRouter::new()
        .nest(
            &settings.url.path,
            ApiRouter::new()
                .with_state(settings.clone())
                .merge(super::openapi::routes(settings.clone()))
                .merge(super::health::routes()),
        )
        .fallback(fallback_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
        )
        .finish_api_with(&mut openapi, |openapi| {
            openapi.default_response::<Json<Value>>()
        })
        .layer(Extension(openapi))
        .into_make_service()
}

pub(super) async fn fallback_handler(
    method: Method,
    OriginalUri(original_uri): OriginalUri,
) -> impl IntoResponse {
    ApiErrorResponse::send(
        StatusCode::NOT_FOUND,
        format!("Can not {}: {}", method, original_uri),
        None,
    )
}
