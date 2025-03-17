use std::sync::Arc;

use async_nats::service::Request;
use serde_json::json;

use crate::common::{state::MessagesState, transform::value_to_bytes};

pub async fn get_messages(_state: Arc<MessagesState>, req: Request) -> anyhow::Result<()> {
    let json = json!({"data": "get_messages"});
    let response = value_to_bytes(&json)?;

    println!("get_messages");

    req.respond(Ok(response)).await?;
    Ok(())
}

pub async fn send_message(
    _state: Arc<MessagesState>,
    req: Request,
) -> anyhow::Result<(), anyhow::Error> {
    let json = json!({"data": "send_messages"});
    let response = value_to_bytes(&json)?;

    println!("send_messages");

    req.respond(Ok(response)).await?;
    Ok(())
}

pub async fn delete_message(
    _state: Arc<MessagesState>,
    req: Request,
) -> anyhow::Result<(), anyhow::Error> {
    let json = json!({"data": "delete_message"});
    let response = value_to_bytes(&json)?;

    println!("delete_message");

    req.respond(Ok(response)).await?;
    Ok(())
}

pub async fn edit_message(
    _state: Arc<MessagesState>,
    req: Request,
) -> anyhow::Result<(), anyhow::Error> {
    let json = json!({"data": "edit_message"});
    let response = value_to_bytes(&json)?;

    println!("edit_message");

    req.respond(Ok(response)).await?;
    Ok(())
}
