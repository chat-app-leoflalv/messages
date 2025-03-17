use core::router::Route;

use async_nats::service::Request;
use common::state::MessagesState;
use message::message_controller;

pub mod common;
pub mod config;
pub mod core;
pub mod message;

pub fn routes<R>(mut router: R) -> R
where
    R: Route<MessagesState, HandlerArgs = Request>,
{
    router.add_handler("message.get_messages", message_controller::get_messages);
    router.add_handler("message.send_message", message_controller::send_message);
    router.add_handler("message.delete_message", message_controller::delete_message);
    router.add_handler("message.edit_message", message_controller::edit_message);

    router
}
