use chrono::{DateTime, NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::db::schema::profiles;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name_enc: Option<Vec<u8>>,
    pub last_name_enc: Option<Vec<u8>>,
    pub phone_enc: Option<Vec<u8>>,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub avatar_url: Option<String>,
    pub status: bool,
    pub social_network: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub user_id: Uuid,
    pub first_name_enc: Option<Vec<u8>>,
    pub last_name_enc: Option<Vec<u8>>,
    pub phone_enc: Option<Vec<u8>>,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub avatar_url: Option<String>,
    pub status: bool,
    pub social_network: Value,
}

impl NewProfile {
    pub fn for_user(user_id: Uuid) -> Self {
        Self {
            user_id,
            first_name_enc: None,
            last_name_enc: None,
            phone_enc: None,
            nickname: None,
            bio: None,
            birthday: None,
            avatar_url: None,
            status: true,
            social_network: serde_json::json!({}),
        }
    }
}
