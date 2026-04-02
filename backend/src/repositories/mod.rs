pub mod base;
pub mod base_repository;
pub mod container;
pub mod impls;
pub mod macros;
pub mod traits;

pub mod audit_logs_repository;
pub mod conversations_repository;
pub mod profiles_repository;
pub mod refresh_tokens_repository;
pub mod users_repository;
pub mod user_keys_repository;

// Re-export key types for convenient access
pub use audit_logs_repository::IAuditLogRepository;
pub use container::AppContainer;
pub use conversations_repository::{
    IConversationMemberRepository, IConversationRepository, IMessageRepository,
};
pub use profiles_repository::IProfileRepository;
pub use refresh_tokens_repository::IRefreshTokenRepository;
pub use users_repository::IUserRepository;
pub use user_keys_repository::IUserKeyRepository;

// ── Manual implementations for all traits ─────────
use crate::db::schema::{audit_logs, profiles, refresh_tokens, users};
use crate::models::audit_log::{AuditLog, NewAuditLog};
use crate::models::profile::{NewProfile, Profile};
use crate::models::refresh_token::{NewRefreshToken, RefreshToken};
use crate::models::user::{NewUser, User};
use crate::repositories::base::BaseRepo;
use chrono::NaiveDateTime;
use diesel::ExpressionMethods;
use ipnet::IpNet;
use uuid::Uuid;

// IUserRepository implementation
#[async_trait::async_trait]
impl IUserRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<User>> {
        use diesel::RunQueryDsl;
        self.run(|conn| users::table.load::<User>(conn)).await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<User> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| users::table.find(id).first::<User>(conn))
            .await
    }

    async fn create(&self, item: &NewUser) -> diesel::QueryResult<User> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewUser) -> diesel::QueryResult<User> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(users::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let search = username_or_email.to_string();
        self.run(move |conn| {
            dsl::users
                .filter(dsl::email.eq(&search))
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn find_by_email(&self, email: &str) -> diesel::QueryResult<Option<User>> {
        self.find_by_username_or_email(email).await
    }

    async fn find_by_reset_token(&self, token: &str) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let token = token.to_string();
        self.run(move |conn| {
            dsl::users
                .filter(dsl::reset_password_token.eq(&token))
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn update_login_info(
        &self,
        id: &Uuid,
        current_sign_in_at: Option<NaiveDateTime>,
        last_sign_in_at: Option<NaiveDateTime>,
        current_sign_in_ip: Option<IpNet>,
        last_sign_in_ip: Option<IpNet>,
    ) -> diesel::QueryResult<User> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::current_sign_in_at.eq(current_sign_in_at),
                    dsl::last_sign_in_at.eq(last_sign_in_at),
                    dsl::current_sign_in_ip.eq(current_sign_in_ip.map(|ip| ip.to_string())),
                    dsl::last_sign_in_ip.eq(last_sign_in_ip.map(|ip| ip.to_string())),
                ))
                .get_result(conn)
        })
        .await
    }

    async fn update_password(
        &self,
        id: &Uuid,
        encrypted_password: &str,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *id;
        let pwd = encrypted_password.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::password_hash.eq(pwd))
                .execute(conn)
        })
        .await
    }

    async fn update_reset_token(
        &self,
        id: &Uuid,
        token: Option<String>,
        sent_at: Option<NaiveDateTime>,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::reset_password_token.eq(token),
                    dsl::reset_password_sent_at.eq(sent_at),
                ))
                .execute(conn)
        })
        .await
    }

    async fn confirm_email(&self, token: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::dsl::now;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let token = token.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.filter(dsl::confirmation_token.eq(&token)))
                .set((
                    dsl::confirmed_at.eq(now),
                    dsl::confirmation_token.eq(None::<String>),
                ))
                .execute(conn)
        })
        .await
    }

    async fn record_failed_login(
        &self,
        user_id: &Uuid,
        _max_attempts: i32,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::failed_attempts.eq(dsl::failed_attempts + 1))
                .execute(conn)
        })
        .await
    }

    async fn record_successful_login(
        &self,
        user_id: &Uuid,
        ip: Option<IpNet>,
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        let ip_str = ip.map(|i| i.to_string());
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::failed_attempts.eq(0),
                    dsl::current_sign_in_ip.eq(ip_str),
                ))
                .execute(conn)
        })
        .await
    }

    async fn get_user_roles(&self, user_id: &Uuid) -> diesel::QueryResult<Vec<String>> {
        use crate::db::schema::{roles, user_roles};
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            user_roles::table
                .inner_join(roles::table)
                .filter(user_roles::user_id.eq(id))
                .select(roles::name)
                .load::<String>(conn)
        })
        .await
    }

    async fn create_password_reset_token(
        &self,
        user_id: &Uuid,
        token: &str,
        sent_at: NaiveDateTime,
    ) -> diesel::QueryResult<usize> {
        self.update_reset_token(user_id, Some(token.to_string()), Some(sent_at))
            .await
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
        let token = token.to_string();
        let pwd = new_password.to_string();
        self.run(move |conn| {
            let user: Option<User> = dsl::users
                .filter(dsl::reset_password_token.eq(&token))
                .first(conn)
                .optional()?;
            if let Some(user) = user {
                diesel::update(dsl::users.find(user.id))
                    .set((
                        dsl::password_hash.eq(pwd),
                        dsl::reset_password_token.eq(None::<String>),
                        dsl::reset_password_sent_at.eq(None::<NaiveDateTime>),
                    ))
                    .execute(conn)
            } else {
                Ok(0)
            }
        })
        .await
    }

    async fn set_otp_secret(&self, user_id: &Uuid, secret: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        let secret = secret.to_string();
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::totp_secret.eq(secret))
                .execute(conn)
        })
        .await
    }

    async fn enable_2fa(
        &self,
        user_id: &Uuid,
        _backup_codes: &[String],
    ) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set(dsl::totp_enabled.eq(true))
                .execute(conn)
        })
        .await
    }

    async fn disable_2fa(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            diesel::update(dsl::users.find(id))
                .set((
                    dsl::totp_secret.eq(None::<String>),
                    dsl::totp_enabled.eq(false),
                ))
                .execute(conn)
        })
        .await
    }
}

// IProfileRepository implementation
#[async_trait::async_trait]
impl IProfileRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Profile>> {
        use diesel::RunQueryDsl;
        self.run(|conn| profiles::table.load::<Profile>(conn)).await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<Profile> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| profiles::table.find(id).first::<Profile>(conn))
            .await
    }

    async fn create(&self, item: &NewProfile) -> diesel::QueryResult<Profile> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(profiles::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewProfile) -> diesel::QueryResult<Profile> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(profiles::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(profiles::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> diesel::QueryResult<Option<Profile>> {
        use crate::db::schema::profiles::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            dsl::profiles
                .filter(dsl::user_id.eq(id))
                .first::<Profile>(conn)
                .optional()
        })
        .await
    }
}

// IRefreshTokenRepository implementation
#[async_trait::async_trait]
impl IRefreshTokenRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<RefreshToken>> {
        use diesel::RunQueryDsl;
        self.run(|conn| refresh_tokens::table.load::<RefreshToken>(conn))
            .await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<RefreshToken> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| refresh_tokens::table.find(id).first::<RefreshToken>(conn))
            .await
    }

    async fn create(&self, item: &NewRefreshToken) -> diesel::QueryResult<RefreshToken> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(refresh_tokens::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewRefreshToken) -> diesel::QueryResult<RefreshToken> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(refresh_tokens::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(refresh_tokens::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> diesel::QueryResult<Option<RefreshToken>> {
        use crate::db::schema::refresh_tokens::dsl;
        use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
        let hash = token_hash.to_string();
        self.run(move |conn| {
            dsl::refresh_tokens
                .filter(dsl::token_hash.eq(&hash))
                .first::<RefreshToken>(conn)
                .optional()
        })
        .await
    }

    async fn revoke(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::refresh_tokens::dsl;
        use chrono::Utc;
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        let now = Utc::now();
        self.run(move |conn| {
            diesel::update(dsl::refresh_tokens.find(id))
                .set(dsl::revoked_at.eq(now))
                .execute(conn)
        })
        .await
    }

    async fn revoke_all_for_user(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        use crate::db::schema::refresh_tokens::dsl;
        use chrono::Utc;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        let now = Utc::now();
        self.run(move |conn| {
            diesel::update(dsl::refresh_tokens.filter(dsl::user_id.eq(id)))
                .set(dsl::revoked_at.eq(now))
                .execute(conn)
        })
        .await
    }
}

// IAuditLogRepository implementation
#[async_trait::async_trait]
impl IAuditLogRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<AuditLog>> {
        use diesel::RunQueryDsl;
        self.run(|conn| audit_logs::table.load::<AuditLog>(conn))
            .await
    }

    async fn find(&self, id: &Uuid) -> diesel::QueryResult<AuditLog> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| audit_logs::table.find(id).first::<AuditLog>(conn))
            .await
    }

    async fn create(&self, item: &NewAuditLog) -> diesel::QueryResult<AuditLog> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(audit_logs::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, id: &Uuid, item: &NewAuditLog) -> diesel::QueryResult<AuditLog> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let id = *id;
        self.run(move |conn| {
            diesel::update(audit_logs::table.find(id))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, id: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let id = *id;
        self.run(move |conn| diesel::delete(audit_logs::table.find(id)).execute(conn))
            .await
    }

    async fn find_by_user(&self, user_id: &Uuid) -> diesel::QueryResult<Vec<AuditLog>> {
        use crate::db::schema::audit_logs::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let id = *user_id;
        self.run(move |conn| {
            dsl::audit_logs
                .filter(dsl::user_id.eq(id))
                .load::<AuditLog>(conn)
        })
        .await
    }

    async fn find_by_action(&self, action: &str) -> diesel::QueryResult<Vec<AuditLog>> {
        use crate::db::schema::audit_logs::dsl;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let act = action.to_string();
        self.run(move |conn| {
            dsl::audit_logs
                .filter(dsl::action.eq(&act))
                .load::<AuditLog>(conn)
        })
        .await
    }
}
