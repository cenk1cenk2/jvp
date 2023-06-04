use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Url {
    pub(crate) path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct OpenApi {
    pub(crate) url: String,
    pub(crate) json: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct RabbitMQ {
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub(crate) port: u16,

    pub(crate) url: Url,
    pub(crate) openapi: OpenApi,
    pub(crate) rabbitmq: RabbitMQ,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::build_config()?.try_deserialize()
    }
}
