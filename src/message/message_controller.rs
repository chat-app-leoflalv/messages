use super::{
    dtos::{
        create_message_request::CreateMessageRequestDto,
        edit_message_request::EditMessageRequestDto,
    },
    message_model::Message,
};

pub struct MessageController {}

impl MessageController {
    pub fn new() -> Self {
        MessageController {}
    }

    pub fn create_message<'a>(self, _req: CreateMessageRequestDto) -> Message<'a> {
        unimplemented!()
    }

    pub fn delete_message<'a>(self, _id: &str) -> Message<'a> {
        unimplemented!()
    }

    pub fn edit_message<'a>(self, _id: &str, _req: EditMessageRequestDto) -> Message<'a> {
        unimplemented!()
    }
}
