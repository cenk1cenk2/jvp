use deadpool::managed::BuildError;
use deadpool_lapin::{Manager, Pool};
use lapin::ConnectionProperties;

pub fn create_rmq_pool(url: &str) -> Result<Pool, BuildError<lapin::Error>> {
    let options = ConnectionProperties::default()
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let manager = Manager::new(url, options);

    deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
}
