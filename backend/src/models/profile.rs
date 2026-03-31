use chrono::{DateTime, NaiveDate, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::db::schema::profiles;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    #[serde(default)]
    #[diesel(sql_type = Uuid)]
    pub id: Uuid,
    #[diesel(sql_type = Uuid)]
    pub user_id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub first_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub last_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub display_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub slug: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub bio: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub avatar_url: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub cover_url: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
    pub birthday: Option<NaiveDate>,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub age_verified: bool,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamptz>)]
    pub age_verified_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub cpf_encrypted: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub cpf_hash: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub phone_encrypted: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub whatsapp_encrypted: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub country: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub state: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Varchar>)]
    pub city: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Jsonb)]
    pub social_network: Value,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub is_creator: bool,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamptz>)]
    pub creator_approved_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub is_agency: bool,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub status: i32,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub created_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub slug: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub social_network: Value,
    pub is_creator: bool,
    pub is_agency: bool,
    pub status: i32,
}

impl NewProfile {
    pub fn for_user(user_id: Uuid) -> Self {
        Self {
            user_id,
            first_name: None,
            last_name: None,
            display_name: None,
            slug: None,
            bio: None,
            birthday: None,
            avatar_url: None,
            cover_url: None,
            social_network: serde_json::json!({}),
            is_creator: false,
            is_agency: false,
            status: 1,
        }
    }
}
