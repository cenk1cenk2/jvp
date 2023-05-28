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
pub struct Settings {
    pub(crate) port: u16,

    pub(crate) url: Url,
    pub(crate) openapi: OpenApi,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::build_config()?.try_deserialize()
    }
}
