use async_nats::service::Request;

#[derive(Clone, Copy)]
pub struct MessageController {}

impl MessageController {
    pub fn new() -> Self {
        MessageController {}
    }

    pub async fn get_messages(&self, req: Request) -> anyhow::Result<()> {
        println!(">>>>");
        println!("{:?}: ", req.message.payload);
        println!(">>>>");
        req.respond(Ok("hello".into())).await?;
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
