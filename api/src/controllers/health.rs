use aide::axum::routing::get_with;
use aide::axum::ApiRouter;

use axum::Json;

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

async fn handler() -> Json<Value> {
    Json(json!({ "test": true }))
}
