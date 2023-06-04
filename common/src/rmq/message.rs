use strum_macros::{AsRefStr, Display, EnumString};

use super::server::RmqRpcServerHandler;

#[derive(AsRefStr, Display, Debug)]
pub enum RmqQueue {
    #[strum(serialize = "amq.rabbitmq.reply-to")]
    ReplyTo,

    Default,
}

#[derive(AsRefStr, Display, Debug)]
pub enum RmqMessage {
    Test,
}

impl RmqRpcServerHandler for RmqMessage {
    fn handle(
        &self,
        delivery: lapin::message::Delivery,
    ) -> anyhow::Result<lapin::message::Delivery> {
        Ok(delivery)
    }
}
