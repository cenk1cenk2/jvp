use std::sync::Arc;

use aide::axum::routing::get_with;
use aide::axum::ApiRouter;

use axum::{Extension, Json};

use crate::prelude::*;
use common::rmq::{client::RmqRpcClient, error::RmqError};
use serde_json::{json, Value};

pub fn routes() -> ApiRouter {
    ApiRouter::new().nest(
        "/health/",
        ApiRouter::new().api_route(
            "/",
            get_with(handler, |o| {
                o.description("test")
                    .response_with::<200, Json<Value>, _>(|res| {
                        res.description("a simple message saying hello to the user")
                            .example(String::from("hello Tom"))
                    })
            }),
        ),
    )
}

async fn handler(
    Extension(ms_calendar_client): Extension<Arc<RmqRpcClient>>,
) -> Result<Json<Value>, ()> {
    info!(
        "{:?}",
        std::str::from_utf8(&ms_calendar_client.send().await.unwrap())
    );

    Ok(Json(json!({ "test": true })))
}
