use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::users;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    #[diesel(deserialize_as = String)]
    pub email: String,
    pub encrypted_password: String,

    // Recoverable
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<DateTime<Utc>>,

    // Rememberable
    pub remember_created_at: Option<DateTime<Utc>>,

    // Trackable
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub current_sign_in_ip: Option<IpNet>,
    pub last_sign_in_ip: Option<IpNet>,

    // Confirmable
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub unconfirmed_email: Option<String>,

    // Lockable
    pub failed_attempts: i32,
    pub unlock_token: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,

    // 2FA (TOTP)
    pub otp_secret: Option<String>,
    pub otp_enabled_at: Option<DateTime<Utc>>,
    pub otp_backup_codes: Option<Vec<Option<String>>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub encrypted_password: String,
    pub confirmation_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(
        email: String,
        encrypted_password: String,
        confirmation_token: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            encrypted_password,
            confirmation_token,
            created_at: now,
            updated_at: now,
        }
    }
}

// #[diesel(postgres_type(name = "inet"))] bug diesel not working type inet postgresql
