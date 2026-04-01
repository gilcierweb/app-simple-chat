use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::message_receipts;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = message_receipts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageReceipt {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub status: i32,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = message_receipts)]
pub struct NewMessageReceipt {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub status: i32,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewMessageReceipt {
    pub fn new(message_id: Uuid, user_id: Uuid, status: i32) -> Self {
        let now = Utc::now();
        Self {
            message_id,
            user_id,
            status,
            delivered_at: None,
            read_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReceiptStatus {
    Delivered = 1,
    Read = 2,
}

impl ReceiptStatus {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for ReceiptStatus {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ReceiptStatus::Delivered),
            2 => Ok(ReceiptStatus::Read),
            _ => Err("Invalid receipt status value"),
        }
    }
}

impl From<ReceiptStatus> for i32 {
    fn from(status: ReceiptStatus) -> Self {
        status.as_i32()
    }
}
