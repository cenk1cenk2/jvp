use crate::prelude::*;
use serde::Deserialize;
use std::env;

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
        let s = config::build_config()?;

        s.try_deserialize()
    }
}
