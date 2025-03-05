use std::future::Future;

use anyhow::Result;
use async_trait::async_trait;

pub mod nats_router;

#[async_trait]
pub trait Router {
    type HandlerArgs;

    async fn connect(server_conn: &str, name: &str, version: &str) -> Result<Self>
    where
        Self: Sized;

    async fn add_handler<F, Fut>(&mut self, route: &'static str, handler: F) -> Result<()>
    where
        F: Fn(Self::HandlerArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send;
}
