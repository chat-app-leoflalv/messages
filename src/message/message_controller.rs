use async_nats::service::Request;
use serde_json::json;

use crate::common::transform::value_to_bytes;

#[derive(Clone, Copy)]
pub struct MessageController {}

impl MessageController {
    pub fn new() -> Self {
        MessageController {}
    }

    pub async fn get_messages(&self, req: Request) -> anyhow::Result<()> {
        let json = json!({"data": "test"});
        let response = value_to_bytes(&json)?;

        req.respond(Ok(response)).await?;
        Ok(())
    }

    pub fn send_message(&self, _req: Request) -> anyhow::Result<(), anyhow::Error> {
        todo!()
    }

    pub fn delete_message(&self, _req: Request) -> anyhow::Result<(), anyhow::Error> {
        todo!()
    }

    pub fn edit_message(&self, _req: Request) -> anyhow::Result<(), anyhow::Error> {
        todo!()
    }
}
