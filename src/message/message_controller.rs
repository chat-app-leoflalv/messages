use async_nats::service::Request;

#[derive(Clone, Copy)]
pub struct MessageController {}

impl MessageController {
    pub fn new() -> Self {
        MessageController {}
    }

    pub fn get_messages(&self, _req: Request) -> anyhow::Result<(), anyhow::Error> {
        todo!()
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
