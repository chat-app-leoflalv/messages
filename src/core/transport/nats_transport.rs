use async_nats::service::{Request, Service, ServiceExt};
use async_trait::async_trait;
use futures::StreamExt;

use super::TransportLayer;

pub struct NatsTransport {
    service: Service,
}

#[async_trait]
impl TransportLayer for NatsTransport {
    type HandlerArgs = Request;

    async fn new(server_path: &str, name: &str, version: &str) -> Result<Self, anyhow::Error> {
        let client = async_nats::connect(server_path).await?;
        let service = client
            .service_builder()
            .start(name, version)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        Ok(NatsTransport { service })
    }

    async fn add_handler<F>(&mut self, route: &'static str, handler: F) -> Result<(), anyhow::Error>
    where
        F: Fn(Self::HandlerArgs) + Send + Sync + 'static,
    {
        let mut endpoint = self
            .service
            .endpoint(route)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        while let Some(request) = endpoint.next().await {
            handler(request);
        }

        Ok(())
    }
}
