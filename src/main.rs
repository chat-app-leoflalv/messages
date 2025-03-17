use messages::core::router::Route;
use messages::{
    common::state::MessagesState, config::envs::Envs, core::router::nats_router::NatsRouter, routes,
};
use tokio::{self, signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let envs = Envs::new();

    let state = MessagesState {};

    let router = NatsRouter::connect(&envs.server_url, "messages", "1.0.0", state).await?;
    let mut router = routes(router);

    router.serve().await?;

    println!("Press Ctrl+C to shutdown");
    signal::ctrl_c().await?;
    println!("Shutdown signal received, cleaning up...");

    println!("Application shutdown gracefully.");

    Ok(())
}
