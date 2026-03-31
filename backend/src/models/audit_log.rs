use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::db::schema::audit_logs;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = audit_logs)]
pub struct AuditLog {
    #[serde(default)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}
