use sqlx::{Database, Pool};

use super::{
    dtos::{
        create_message_request::CreateMessageRequestDto,
        edit_message_request::EditMessageRequestDto,
    },
    message_model::Message,
};

pub struct MessageService<T: Database> {
    db_conn: Pool<T>,
}

impl<T: Database> MessageService<T> {
    pub fn new(db_conn: Pool<T>) -> Self {
        MessageService { db_conn }
    }

    pub fn create_message<'a>(self, _req: CreateMessageRequestDto) -> Message<'a> {
        todo!()
    }

    pub fn delete_message<'a>(self, _id: &str) -> Message<'a> {
        todo!()
    }

    pub fn edit_message<'a>(self, _id: &str, _req: EditMessageRequestDto) -> Message<'a> {
        todo!()
    }
}
