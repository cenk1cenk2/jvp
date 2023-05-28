use crate::prelude::*;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct RabbitMQ {
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub(crate) rabbitmq: RabbitMQ,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::build_config()?.try_deserialize()
    }
}
