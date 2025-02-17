use chrono::{DateTime, Utc};

use super::message_status::Status;

#[derive(sqlx::FromRow)]
pub struct Message<'a> {
    pub id: &'a str,
    pub text: &'a str,
    pub status: Status,
    pub timestamp: DateTime<Utc>,

    pub user_id: &'a str,
    pub chat_id: &'a str,
}
