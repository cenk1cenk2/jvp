use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod controllers;
mod error;
mod prelude;
mod response;
mod states;

use crate::controllers::root::routes;
use crate::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .init();

    let settings = Arc::new(match Settings::new() {
        Ok(config) => config,
        Err(err) => panic!("Can not read configuration: {}", err),
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    info!("Starting application on: {}", addr);

    axum::Server::bind(&addr)
        .serve(routes(settings))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()))
}
