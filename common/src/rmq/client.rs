use futures::{Future, StreamExt};
use uuid::Uuid;

use super::{error::RmqError, message::RmqQueue, server::RmqRpcServer};
use deadpool_lapin::{Manager, Pool};

use lapin::{
    message::Delivery, options::*, types::FieldTable, BasicProperties, Channel,
    ConnectionProperties, Consumer,
};
use tokio::sync::Mutex;

use std::{
    collections::BTreeMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration,
};

use crate::logger::*;

#[derive(Debug)]
pub struct RmqRpcClient {
    pool: Pool,
    replies: Replies,
    server: RmqRpcServer,
}

type Replies = Arc<Mutex<BTreeMap<String, FutureRpcReply<Vec<u8>>>>>;

impl RmqRpcClient {
    pub async fn new(pool: Pool) -> Result<Self, RmqError> {
        let server = RmqRpcServer::new(pool.clone(), RmqQueue::ReplyTo.to_string());

        let replies: Replies = Arc::new(Mutex::new(BTreeMap::new()));

        let s = Self {
            server,
            pool,
            replies,
        };

        tokio::spawn(async move {
            s.handle_results().await;
        });

        Ok(s)
    }

    pub async fn send(&self) -> Result<Vec<u8>, RmqError> {
        let correlation_id = Uuid::new_v4();

        self.server
            .channel
            .unwrap()
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                &Vec::from("test"),
                BasicProperties::default()
                    .with_reply_to("amq.rabbitmq.reply-to".into())
                    .with_correlation_id(correlation_id.to_string().into()),
            )
            .await?;

        let reply = FutureRpcReply::new();

        self.replies
            .lock()
            .await
            .insert(correlation_id.to_string(), reply.clone());

        Ok(reply.await)
    }

    async fn handle_results(&self) -> Result<(), RmqError> {
        let mut consumer = self
            .server
            .create_consumer(
                "amq.rabbitmq.reply-to",
                BasicConsumeOptions {
                    no_ack: true,
                    ..BasicConsumeOptions::default()
                },
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    match delivery.properties.correlation_id() {
                        Some(v) => match self.replies.lock().await.remove(&v.to_string()) {
                            Some(v) => v.resolve(delivery.data).await,
                            None => unimplemented!(),
                        },
                        None => unimplemented!(),
                    };
                }
                Err(e) => error!("{:?}", e),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct SharedState<T: Clone> {
    delivery: Option<T>,
    waker: Option<Waker>,
}

#[derive(Clone, Debug)]
pub struct FutureRpcReply<T: Clone + std::fmt::Debug> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<T: Clone + std::fmt::Debug> FutureRpcReply<T> {
    pub fn new() -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            delivery: None,
            waker: None,
        }));

        Self { shared_state }
    }

    pub async fn resolve(self, delivery: T) {
        let mut shared_state = self.shared_state.lock().await;
        shared_state.delivery = Some(delivery);

        match shared_state.waker.clone() {
            Some(waker) => waker.wake(),
            None => panic!("Future has never awaited before!"),
        }
    }
}

impl<T: Clone + std::fmt::Debug> Default for FutureRpcReply<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + std::fmt::Debug> Future for FutureRpcReply<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut lock_fut = self.shared_state.lock();
        let lock = unsafe { Pin::new_unchecked(&mut lock_fut) };

        match lock.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(mut state) => {
                state.waker = Some(cx.waker().clone());

                match state.delivery.clone() {
                    Some(v) => Poll::Ready(v),
                    None => Poll::Pending,
                }
            }
        }
    }
}
