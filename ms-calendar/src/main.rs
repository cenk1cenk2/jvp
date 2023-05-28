mod prelude;
mod settings;

use std::sync::Arc;

use crate::prelude::*;

fn main() {
    logger::setup_tracing();

    let _settings = Arc::new(match Settings::new() {
        Ok(config) => config,
        Err(err) => panic!("Can not read configuration: {}", err),
    });

    println!("imdat!");

    todo!()
}
