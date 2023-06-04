mod config;
mod controllers;
mod prelude;

use common::rmq::{
    message::RmqQueue,
    pool::create_rmq_pool,
    server::{Delivery, RmqRpcServer, RmqRpcServerHandler},
};
use std::sync::Arc;

use crate::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::setup_tracing();

    let config = Arc::new(match Config::new() {
        Ok(config) => config,
        Err(err) => panic!("Can not read configuration: {}", err),
    });

    let rmq_pool = create_rmq_pool(&config.rabbitmq.url.clone())?;

    let rmq_server = RmqRpcServer::new(rmq_pool, RmqQueue::Default.into());

    rmq_server.listen(RmqServerRequests).await?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub enum RmqServerRequests {
    Test,
}

impl RmqRpcServerHandler for RmqServerRequests {
    fn handle(&self, delivery: Delivery) -> anyhow::Result<Delivery> {
        Ok(delivery)
    }
}
