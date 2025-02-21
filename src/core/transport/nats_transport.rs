use anyhow::Result;
use async_nats::{Client, Message};
use async_trait::async_trait;

use futures::StreamExt;

use super::TransportLayer;

pub struct NatsTransport {
    client: Client,
}

#[async_trait]
impl TransportLayer for NatsTransport {
    type HandlerArgs = Message;

    async fn new(server_path: &str) -> Result<Self> {
        let client = async_nats::connect(server_path).await?;
        Ok(NatsTransport { client })
    }

    async fn add_handler<F>(&mut self, route: &'static str, handler: F) -> Result<()>
    where
        F: Fn(Self::HandlerArgs) + Send + Sync + 'static,
    {
        let mut subscriber = self.client.subscribe(route).await?;

        while let Some(message) = subscriber.next().await {
            handler(message);
        }

        Ok(())
    }
}
