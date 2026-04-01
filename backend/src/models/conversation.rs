use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::conversations;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    pub id: Uuid,
    pub conversation_type: i32,
    pub name_enc: Option<Vec<u8>>,
    pub avatar_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub id: Uuid,
    pub conversation_type: i32,
    pub name_enc: Option<Vec<u8>>,
    pub avatar_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewConversation {
    pub fn new(conversation_type: i32, created_by: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            conversation_type,
            name_enc: None,
            avatar_url: None,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_name(mut self, name_enc: Vec<u8>) -> Self {
        self.name_enc = Some(name_enc);
        self
    }

    pub fn with_avatar(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ConversationType {
    Direct = 1,
    Group = 2,
}

impl ConversationType {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for ConversationType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ConversationType::Direct),
            2 => Ok(ConversationType::Group),
            _ => Err("Invalid conversation type value"),
        }
    }
}

impl From<ConversationType> for i32 {
    fn from(conversation_type: ConversationType) -> Self {
        conversation_type.as_i32()
    }
}
