use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::user_keys;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = user_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_type: i32,
    pub public_key: String,
    pub signature: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub used_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = user_keys)]
pub struct NewUserKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_type: i32,
    pub public_key: String,
    pub signature: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub used_at: Option<DateTime<Utc>>,
}

impl NewUserKey {
    pub fn new(
        user_id: Uuid,
        key_type: i32,
        public_key: String,
        signature: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            key_type,
            public_key,
            signature,
            created_at: now,
            updated_at: now,
            expires_at: None,
            used_at: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum KeyType {
    Identity = 1,
    SignedPrekey = 2,
    OneTimePrekey = 3,
}

impl KeyType {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for KeyType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KeyType::Identity),
            2 => Ok(KeyType::SignedPrekey),
            3 => Ok(KeyType::OneTimePrekey),
            _ => Err("Invalid key type value"),
        }
    }
}

impl From<KeyType> for i32 {
    fn from(key_type: KeyType) -> Self {
        key_type.as_i32()
    }
}
