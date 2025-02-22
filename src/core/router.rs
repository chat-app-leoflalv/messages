use anyhow::Result;
use async_trait::async_trait;

pub mod nats_router;

#[async_trait]
pub trait Router {
    type HandlerArgs;

    async fn new(server_conn: &str, name: &str, version: &str) -> Result<Self>
    where
        Self: Sized;

    async fn add_handler<F>(&mut self, route: &'static str, handler: F) -> Result<()>
    where
        F: Fn(Self::HandlerArgs) -> Result<(), anyhow::Error> + Send + Sync + 'static;
}
