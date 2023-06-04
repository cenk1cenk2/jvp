use thiserror::Error;

#[derive(Error, Debug)]
pub enum RmqError {
    #[error("RMQ client error: {0}")]
    Lapin(#[from] lapin::Error),

    #[error("RMQ connection pool error: {0}")]
    Pool(#[from] deadpool_lapin::PoolError),

    #[error("RMQ can not build connection pool: {0}")]
    PoolBuilder(#[from] deadpool::managed::BuildError<lapin::Error>),
}
