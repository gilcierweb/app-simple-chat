use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::conversation_members;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = conversation_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConversationMember {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: i32,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = conversation_members)]
pub struct NewConversationMember {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: i32,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewConversationMember {
    pub fn new(conversation_id: Uuid, user_id: Uuid, role: i32) -> Self {
        let now = Utc::now();
        Self {
            conversation_id,
            user_id,
            role,
            joined_at: now,
            last_read_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MemberRole {
    Owner = 1,
    Admin = 2,
    Member = 3,
}

impl MemberRole {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for MemberRole {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MemberRole::Owner),
            2 => Ok(MemberRole::Admin),
            3 => Ok(MemberRole::Member),
            _ => Err("Invalid member role value"),
        }
    }
}

impl From<MemberRole> for i32 {
    fn from(role: MemberRole) -> Self {
        role.as_i32()
    }
}
