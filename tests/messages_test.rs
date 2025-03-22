use std::time::Duration;

use bytes::Bytes;
use messages::start_serve;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::nats;
use tokio::time::sleep;

#[tokio::test]
async fn should_get_messages() {
    let container = nats::Nats::default().start().await.unwrap();
    let port = container.get_host_port_ipv4(4222).await.unwrap();

    // prepare connection string
    let connection_url = format!("nats://localhost:{}", port);
    std::env::set_var("NATS_SERVERS", &connection_url);

    sleep(Duration::from_secs(1)).await;
    tokio::spawn(start_serve());

    let client = async_nats::connect(connection_url).await.unwrap();
    sleep(Duration::from_secs(2)).await;

    let response = client
        .request("messages.get_messages", Bytes::default())
        .await
        .unwrap();
}
