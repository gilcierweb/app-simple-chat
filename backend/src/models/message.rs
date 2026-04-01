use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::messages;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub ciphertext: Vec<u8>,
    pub iv: String,
    pub message_type: i32,
    pub reply_to_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub ciphertext: Vec<u8>,
    pub iv: String,
    pub message_type: i32,
    pub reply_to_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl NewMessage {
    pub fn new(
        conversation_id: Uuid,
        sender_id: Uuid,
        ciphertext: Vec<u8>,
        iv: String,
        message_type: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender_id,
            ciphertext,
            iv,
            message_type,
            reply_to_id: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn with_reply_to(mut self, reply_to_id: Uuid) -> Self {
        self.reply_to_id = Some(reply_to_id);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageType {
    Text = 1,
    Image = 2,
    File = 3,
    Audio = 4,
}

impl MessageType {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for MessageType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageType::Text),
            2 => Ok(MessageType::Image),
            3 => Ok(MessageType::File),
            4 => Ok(MessageType::Audio),
            _ => Err("Invalid message type value"),
        }
    }
}

impl From<MessageType> for i32 {
    fn from(message_type: MessageType) -> Self {
        message_type.as_i32()
    }
}
