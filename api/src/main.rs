use std::net::SocketAddr;
use std::sync::Arc;

mod controllers;
mod error;
mod prelude;
mod response;
mod settings;
mod states;

use crate::controllers::root::routes;
use crate::prelude::*;

#[tokio::main]
async fn main() {
    logger::setup_tracing();

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
