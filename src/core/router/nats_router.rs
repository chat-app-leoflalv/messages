use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use async_nats::service::{endpoint::Endpoint, Request, Service, ServiceExt};
use async_trait::async_trait;
use futures::{future::join_all, StreamExt};
use tokio::task::JoinHandle;

use crate::common::types::SafeState;

use super::Route;

pub type Handler<S> = Arc<
    dyn Fn(Arc<S>, Request) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>> + Send>>
        + Send
        + Sync,
>;

pub struct NatsRouter<S> {
    service: Service,
    state: Arc<S>,
    routes: Vec<(String, Handler<S>)>,
    tasks: Vec<JoinHandle<()>>,
}

#[async_trait]
impl<S> Route<S> for NatsRouter<S>
where
    S: SafeState,
{
    type HandlerArgs = Request;

    async fn connect(
        server_path: &str,
        name: &str,
        version: &str,
        shared_state: S,
    ) -> Result<Self, anyhow::Error> {
        let client = async_nats::connect(server_path).await?;
        let service = client
            .service_builder()
            .start(name, version)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let state = Arc::new(shared_state);

        Ok(NatsRouter {
            service,
            state,
            routes: Vec::new(),
            tasks: Vec::new(),
        })
    }

    fn add_handler<F, Fut>(&mut self, route: &str, handler: F)
    where
        F: Fn(Arc<S>, Self::HandlerArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    {
        let boxed_handler: Handler<S> = Arc::new(move |state, req| Box::pin(handler(state, req)));

        self.routes.push((route.to_string(), boxed_handler));
    }

    async fn serve(&mut self) -> anyhow::Result<()> {
        for (route, handler) in self.routes.iter_mut() {
            let state = Arc::clone(&self.state);
            let handler = Arc::clone(handler);

            let mut endpoint = self
                .service
                .endpoint(route)
                .await
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            let task = tokio::spawn(async move {
                while let Some(request) = endpoint.next().await {
                    let _ = handler(Arc::clone(&state), request).await;
                }
            });

            self.tasks.push(task);
        }

        join_all(std::mem::take(&mut self.tasks)).await;

        Ok(())
    }

    async fn stop(self) -> anyhow::Result<()> {
        for task in &self.tasks {
            task.abort();
        }

        let _ = join_all(self.tasks).await;

        self.service
            .stop()
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        Ok(())
    }
}
