use crate::prelude::*;
use aide::axum::ApiRouter;
use axum::response::IntoResponse;
use axum::routing::IntoMakeService;
use axum::{
    extract::OriginalUri,
    http::{Method, StatusCode},
};
use axum::{Extension, Json, Router};
use common::rmq::client::RmqRpcClient;
use common::rmq::pool::create_rmq_pool;
use serde_json::Value;

use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};

use super::openapi::generate;

pub async fn routes(config: Arc<Config>) -> anyhow::Result<IntoMakeService<Router>> {
    let mut openapi = generate();
    let rmq_pool = create_rmq_pool(&config.rabbitmq.url.clone()).unwrap();

    let ms_calendar_client = Arc::new(RmqRpcClient::new(rmq_pool).await?);

    Ok(ApiRouter::new()
        .nest(
            &config.url.path,
            ApiRouter::new()
                .with_state(config.clone())
                .merge(super::openapi::routes(config.clone()))
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
        .layer(Extension(ms_calendar_client))
        .into_make_service())
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
