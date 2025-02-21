use anyhow::Result;
use async_trait::async_trait;

pub mod nats_transport;

#[async_trait]
pub trait TransportLayer {
    type HandlerArgs;

    async fn new(server_conn: &str) -> Result<Self>
    where
        Self: Sized;

    async fn add_handler<F>(&mut self, route: &'static str, handler: F) -> Result<()>
    where
        F: Fn(Self::HandlerArgs) + Send + Sync + 'static;
}
