use async_trait::async_trait;
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::schema::user_keys;
use crate::models::user_key::{KeyType, NewUserKey, UserKey};
use crate::repositories::base::BaseRepo;
use crate::repositories::user_keys_repository::IUserKeyRepository;

#[async_trait]
impl IUserKeyRepository for BaseRepo {
    async fn insert_keys(&self, keys: Vec<NewUserKey>) -> diesel::QueryResult<usize> {
        use diesel::RunQueryDsl;

        self.run(move |conn| {
            diesel::insert_into(user_keys::table)
                .values(&keys)
                .execute(conn)
        })
        .await
    }

    async fn fetch_key_bundle(
        &self,
        target_user_id: Uuid,
    ) -> diesel::QueryResult<Option<(UserKey, UserKey, Option<UserKey>)>> {
        use diesel::{ExpressionMethods, OptionalExtension};
        use crate::db::schema::user_keys::dsl;

        self.run(move |conn| {
            // Fetch Identity key
            let identity_res = dsl::user_keys
                .filter(dsl::user_id.eq(target_user_id))
                .filter(dsl::key_type.eq(KeyType::Identity.as_i32()))
                .order(dsl::created_at.desc())
                .first::<UserKey>(conn)
                .optional()?;

            let identity = match identity_res {
                Some(k) => k,
                None => return Ok(None),
            };

            // Fetch Signed PreKey
            let signed_res = dsl::user_keys
                .filter(dsl::user_id.eq(target_user_id))
                .filter(dsl::key_type.eq(KeyType::SignedPrekey.as_i32()))
                .order(dsl::created_at.desc())
                .first::<UserKey>(conn)
                .optional()?;

            let signed = match signed_res {
                Some(k) => k,
                None => return Ok(None),
            };

            // Fetch One OneTime PreKey (unused)
            let one_time_res = dsl::user_keys
                .filter(dsl::user_id.eq(target_user_id))
                .filter(dsl::key_type.eq(KeyType::OneTimePrekey.as_i32()))
                .filter(dsl::used_at.is_null())
                .order(dsl::created_at.asc())
                .first::<UserKey>(conn)
                .optional()?;

            let mut final_one_time = None;

            if let Some(otk) = one_time_res {
                // Mark it as used
                diesel::update(dsl::user_keys.find(otk.id))
                    .set(dsl::used_at.eq(Utc::now()))
                    .execute(conn)?;
                
                final_one_time = Some(otk);
            }

            Ok(Some((identity, signed, final_one_time)))
        })
        .await
    }

    async fn get_identity_key(&self, target_user_id: Uuid) -> diesel::QueryResult<Option<UserKey>> {
        use diesel::{ExpressionMethods, OptionalExtension};
        use crate::db::schema::user_keys::dsl;

        self.run(move |conn| {
            dsl::user_keys
                .filter(dsl::user_id.eq(target_user_id))
                .filter(dsl::key_type.eq(KeyType::Identity.as_i32()))
                .order(dsl::created_at.desc())
                .first::<UserKey>(conn)
                .optional()
        })
        .await
    }
}
