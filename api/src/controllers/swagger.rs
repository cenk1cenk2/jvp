use axum::{response::Html, routing::get, Extension, Router};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct SwaggerUiAssets;

pub struct SwaggerUi;

impl SwaggerUi {
    pub fn setup(spec_url: String) -> Router {
        Router::new()
            .route("/", get(index))
            .layer(Extension(spec_url))
    }
}

async fn index(Extension(spec_url): Extension<String>) -> Html<String> {
    let html = std::str::from_utf8(SwaggerUiAssets::get("swagger.html").unwrap().data.as_ref())
        .unwrap()
        .replace("{:spec_url}", &spec_url);

    Html(html)
}
