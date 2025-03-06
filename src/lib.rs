use core::router::Router;

use async_nats::service::Request;
use message::message_controller::MessageController;

pub mod common;
pub mod config;
pub mod core;
pub mod message;

pub async fn routes<R>(mut router: R) -> anyhow::Result<()>
where
    R: Router<HandlerArgs = Request> + 'static,
{
    let controller = MessageController::new();

    router
        .add_handler("message.get_messages", move |req| async move {
            controller.get_messages(req).await
        })
        .await?;

    Ok(())
}
