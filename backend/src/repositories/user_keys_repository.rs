use async_trait::async_trait;
use diesel::QueryResult;
use uuid::Uuid;

use crate::models::user_key::{NewUserKey, UserKey};

#[async_trait]
pub trait IUserKeyRepository: Send + Sync {
    /// Batch insert keys (Identity, Signed, One-Time)
    async fn insert_keys(&self, keys: Vec<NewUserKey>) -> QueryResult<usize>;

    /// Fetch a prekey bundle for a user.
    /// Returns (IdentityKey, SignedPreKey, Option<OneTimePreKey>).
    /// If a OneTimePreKey is provided, it must be marked as used in the database.
    async fn fetch_key_bundle(
        &self,
        user_id: Uuid,
    ) -> QueryResult<Option<(UserKey, UserKey, Option<UserKey>)>>;
    
    /// Get identity key of a user
    async fn get_identity_key(&self, user_id: Uuid) -> QueryResult<Option<UserKey>>;
}
