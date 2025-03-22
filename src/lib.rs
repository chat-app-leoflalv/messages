use core::router::{nats_router::NatsRouter, Route};

use async_nats::service::Request;
use common::state::MessagesState;
use config::envs::Envs;
use message::message_controller;

pub mod common;
pub mod config;
pub mod core;
pub mod message;

pub fn routes<R>(mut router: R) -> R
where
    R: Route<MessagesState, HandlerArgs = Request>,
{
    router.add_handler("messages.get_messages", message_controller::get_messages);
    router.add_handler("messages.send_message", message_controller::send_message);
    router.add_handler(
        "messages.delete_message",
        message_controller::delete_message,
    );
    router.add_handler("messages.edit_message", message_controller::edit_message);

    router
}

pub async fn start_serve() -> anyhow::Result<()> {
    let envs = Envs::new();

    let state = MessagesState {};

    println!("envs: {:?}", envs);

    let router = NatsRouter::connect(&envs.server_url, "messages", "1.0.0", state).await?;
    let mut router = routes(router);

    router.serve().await?;

    Ok(())
}
