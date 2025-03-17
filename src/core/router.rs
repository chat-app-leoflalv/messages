use std::{future::Future, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;

use crate::common::types::SafeState;

pub mod nats_router;

#[async_trait]
pub trait Route<S: SafeState> {
    type HandlerArgs;

    async fn connect(server_conn: &str, name: &str, version: &str, shared_state: S) -> Result<Self>
    where
        Self: Sized;

    fn add_handler<F, Fut>(&mut self, route: &'static str, handler: F)
    where
        F: Fn(Arc<S>, Self::HandlerArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static;

    async fn serve(&mut self) -> Result<()>;
}
