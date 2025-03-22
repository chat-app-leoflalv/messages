use messages::start_serve;
use tokio::{self, signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_serve().await?;

    println!("Press Ctrl+C to shutdown");
    signal::ctrl_c().await?;
    println!("Shutdown signal received, cleaning up...");

    println!("Application shutdown gracefully.");

    Ok(())
}
