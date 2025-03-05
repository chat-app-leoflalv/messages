use std::future::Future;

use anyhow::Result;
use async_nats::service::{Request, Service, ServiceExt};
use async_trait::async_trait;
use futures::StreamExt;

use super::Router;

pub struct NatsRouter {
    pub service: Service,
}

#[async_trait]
impl Router for NatsRouter {
    type HandlerArgs = Request;

    async fn connect(server_path: &str, name: &str, version: &str) -> Result<Self, anyhow::Error> {
        let client = async_nats::connect(server_path).await?;
        let service = client
            .service_builder()
            .start(name, version)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        Ok(NatsRouter { service })
    }

    async fn add_handler<F, Fut>(
        &mut self,
        route: &'static str,
        handler: F,
    ) -> Result<(), anyhow::Error>
    where
        F: Fn(Self::HandlerArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send,
    {
        let mut endpoint = self
            .service
            .endpoint(route)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        while let Some(request) = endpoint.next().await {
            handler(request).await?;
        }

        Ok(())
    }
}
