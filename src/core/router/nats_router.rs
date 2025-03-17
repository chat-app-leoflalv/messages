use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use async_nats::service::{endpoint::Endpoint, Request, ServiceExt};
use async_trait::async_trait;
use futures::StreamExt;

use crate::common::types::SafeState;

use super::Route;

pub type Handler<S> = Arc<
    dyn Fn(Arc<S>, Request) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>> + Send>>
        + Send
        + Sync,
>;

pub struct NatsRouter<S> {
    endpoint: Endpoint,
    state: Arc<S>,
    routes: HashMap<&'static str, Handler<S>>,
}

impl<S: SafeState> NatsRouter<S> {
    async fn handle_request(
        request: Request,
        state: Arc<S>,
        routes: Arc<&HashMap<&'static str, Handler<S>>>,
    ) -> anyhow::Result<()> {
        let route = request.message.subject.as_str();

        if let Some(handler) = routes.get(route) {
            let handler = Arc::clone(handler);
            tokio::spawn(async move {
                let _ = handler(state, request).await;
            });
        }

        Ok(())
    }
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

        let endpoint = service
            .endpoint("messages")
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let state = Arc::new(shared_state);
        let routes = HashMap::new();

        Ok(NatsRouter {
            endpoint,
            state,
            routes,
        })
    }

    fn add_handler<F, Fut>(&mut self, route: &'static str, handler: F)
    where
        F: Fn(Arc<S>, Self::HandlerArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    {
        let boxed_handler: Handler<S> = Arc::new(move |state, req| Box::pin(handler(state, req)));

        self.routes.insert(route, boxed_handler);
    }

    async fn serve(&mut self) -> anyhow::Result<()> {
        while let Some(request) = self.endpoint.next().await {
            let state = Arc::clone(&self.state);
            let routes = Arc::new(&self.routes);
            let _ = NatsRouter::handle_request(request, state, routes).await;
        }

        Ok(())
    }
}
