use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod controllers;
mod error;
mod prelude;
mod response;
mod states;

use crate::controllers::root::routes;
use crate::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::setup_tracing();

    let config = Arc::new(match Config::new() {
        Ok(config) => config,
        Err(err) => panic!("Can not read configuration: {}", err),
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Starting application on: {}", addr);

    axum::Server::bind(&addr)
        .serve(routes(config).await?)
        .await?;

    Ok(())
}
