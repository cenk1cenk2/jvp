use anyhow::anyhow;
use async_trait::async_trait;
use futures::StreamExt;
pub use lapin::message::Delivery;
use lapin::Consumer;

use super::error::RmqError;
use deadpool_lapin::Pool;
use uuid::Uuid;

use lapin::{options::*, types::FieldTable, BasicProperties, Channel};

use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::logger::*;

#[derive(Debug)]
pub struct RmqRpcServer {
    pub queue_name: String,
    pool: Pool,
}

impl RmqRpcServer {
    pub fn new(pool: Pool, queue_name: String) -> Self {
        Self {
            pool,
            queue_name,
            ..Self::default()
        }
    }

    pub async fn listen(&self, handler: &impl RmqRpcServerHandler) -> Result<(), RmqError> {
        let mut retry_interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            retry_interval.tick().await;

            let channel = self.create_channel().await?;
            match self.handle(channel, handler).await {
                Ok(_) => info!("Connected to the RMQ pool."),
                Err(e) => error!("{}", e),
            };
        }
    }

    async fn create_channel(&self) -> Result<Channel, RmqError> {
        let pool = self.pool.get().await?;
        let channel = pool.create_channel().await?;

        Ok(channel)
    }

    async fn handle(
        &self,
        channel: Channel,
        handler: &impl RmqRpcServerHandler,
    ) -> Result<(), RmqError> {
        let queue = channel
            .queue_declare(
                &self.queue_name.clone(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let consumer_tag = &format!("{}-{}", "lapin", Uuid::new_v4());

        let mut consumer = channel
            .basic_consume(
                &self.queue_name.clone(),
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "Consuming RMQ queue: {} -> {} ",
            consumer_tag,
            &self.queue_name.clone()
        );

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    // let data = String::from_utf8_lossy(delivery.data.as_slice());

                    let routing_key = delivery.routing_key.as_str();

                    debug!(
                        "Received message from queue: {} -> {}",
                        queue.name().as_str(),
                        routing_key,
                    );

                    let (reply_to, correlation_id) = (
                        delivery.properties.reply_to().clone(),
                        delivery.properties.correlation_id().clone(),
                    );

                    match handler.handle(delivery) {
                        Ok(delivery) => {
                            if let (Some(reply_to), Some(correlation_id)) =
                                (reply_to, correlation_id)
                            {
                                debug!("Should reply to: {} -> {}", reply_to, correlation_id);

                                let publish = channel
                                    .basic_publish(
                                        "",
                                        reply_to.as_str(),
                                        BasicPublishOptions::default(),
                                        &Vec::from("yattara"),
                                        BasicProperties::default()
                                            .with_correlation_id(correlation_id.clone()),
                                    )
                                    .await;

                                match publish {
                                    Ok(_) => {
                                        debug!(
                                            "Published the results: {} -> {}",
                                            reply_to, correlation_id
                                        )
                                    }
                                    Err(e) => {
                                        warn!(
                                    "Error when publishing reply to routing key: {} -> {} -> {:?}",
                                    reply_to, correlation_id, e
                                )
                                    }
                                };
                            }

                            delivery.ack(BasicAckOptions::default()).await?;
                        }
                        // Err(err) => delivery.nack(BasicNackOptions::default()).await?,
                        Err(_) => unimplemented!(),
                    }
                }
                Err(err) => error!("Can not handle: {}", err),
            }
        }

        Ok(())
    }
}

impl Default for RmqRpcServer {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub trait RmqRpcServerHandler {
    fn handle(&self, delivery: Delivery) -> anyhow::Result<Delivery>;
}
