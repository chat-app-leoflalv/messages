use messages::{
    config::envs::Envs,
    core::router::{nats_router::NatsRouter, Router},
    routes,
};
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let envs = Envs::new();

    let router = NatsRouter::connect(&envs.server_url, "messages", "1.0.0").await?;
    routes(router).await?;

    Ok(())
}
