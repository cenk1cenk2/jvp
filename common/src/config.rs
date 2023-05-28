pub use config::{Config, ConfigError};
use config::{Environment, File};
use std::env;

pub fn build_config() -> Result<Config, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name("config/default"))
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        .add_source(File::with_name("config/local").required(false))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("app"))
        .build()
}
