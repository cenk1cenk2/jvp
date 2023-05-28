use crate::prelude::*;
use std::sync::Arc;

use aide::openapi::{Info, OpenApi};
use axum::{routing::get, Extension, Json, Router};

use super::swagger::SwaggerUi;

pub fn routes(settings: Arc<Settings>) -> Router {
    let json = settings.openapi.json.clone();

    Router::new()
        .route(&json, get(serve))
        .nest(&settings.openapi.url, SwaggerUi::setup(json))
}

async fn serve(Extension(openapi): Extension<OpenApi>) -> Json<OpenApi> {
    Json(openapi)
}

pub fn generate() -> OpenApi {
    aide::gen::on_error(|error| {
        panic!("{}", error);
    });

    aide::gen::extract_schemas(true);

    OpenApi {
        openapi: "3.0.0",
        info: Info {
            title: "Je Vous Piste".to_string(),
            description: Some(
                "Rendezvous and scheduling application for professionals.".to_string(),
            ),
            ..Info::default()
        },
        ..OpenApi::default()
    }
}
