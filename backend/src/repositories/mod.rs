pub mod base;
pub mod base_repository;
pub mod container;
pub mod macros;

pub mod audit_logs_repository;
pub mod creator_settings_repository;
pub mod kyc_verifications_repository;
pub mod live_streams_impl;
pub mod live_streams_repository;
pub mod media_repository;
pub mod messages_impl;
pub mod messages_repository;
pub mod notification_preferences_impl;
pub mod notification_preferences_repository;
pub mod notifications_impl;
pub mod notifications_repository;
pub mod posts_repository;
pub mod ppv_purchases_repository;
pub mod profiles_repository;
pub mod refresh_tokens_repository;
pub mod reports_repository;
pub mod tips_repository;
pub mod user_roles_repository;
pub mod users_repository;
pub mod withdrawals_repository;

// Re-export key types for convenient access
pub use audit_logs_repository::IAuditLogRepository;
pub use container::AppContainer;
pub use creator_settings_repository::ICreatorSettingRepository;
pub use kyc_verifications_repository::IKycVerificationRepository;
pub use live_streams_repository::ILiveStreamRepository;
pub use media_repository::IMediaRepository;
pub use messages_repository::IMessageRepository;
pub use posts_repository::IPostRepository;
pub use ppv_purchases_repository::IPpvPurchaseRepository;
pub use profiles_repository::IProfileRepository;
pub use refresh_tokens_repository::IRefreshTokenRepository;
pub use reports_repository::IReportRepository;
pub use tips_repository::ITipRepository;
pub use user_roles_repository::IUserRoleRepository;
pub use users_repository::IUserRepository;
pub use withdrawals_repository::IWithdrawalRepository;

// -- Macro-generated CRUD implementations on BaseRepo
// These must be here (after `mod macros` is compiled) so the
// `impl_crud!` macro and all schema/model types are in scope.

use crate::db::schema::audit_logs as audit_logs_table;
use crate::db::schema::creator_settings as creator_settings_table;
use crate::db::schema::kyc_verifications as kyc_verifications_table;
use crate::db::schema::posts as posts_table;
use crate::db::schema::ppv_purchases as ppv_purchases_table;
use crate::db::schema::profiles as profiles_table;
use crate::db::schema::refresh_tokens as refresh_tokens_table;
use crate::db::schema::reports as reports_table;
use crate::db::schema::tips as tips_table;
use crate::db::schema::users as users_table;
use crate::db::schema::users_roles as users_roles_table;
use crate::db::schema::withdrawals as withdrawals_table;
use crate::models::audit_log::{AuditLog, NewAuditLog};
use crate::models::creator_setting::{CreatorSetting, NewCreatorSetting};
use crate::models::kyc_verification::{KycVerification, NewKycVerification};
use crate::models::post::{NewPost, Post};
use crate::models::ppv_purchase::{NewPpvPurchase, PpvPurchase};
use crate::models::profile::{NewProfile, Profile};
use crate::models::refresh_token::{NewRefreshToken, RefreshToken};
use crate::models::report::{NewReport, Report};
use crate::models::tip::{NewTip, Tip};
use crate::models::user::{NewUser, User};
use crate::models::user_role::{NewUserRole, UserRole};
use crate::models::withdrawals::{NewWithdrawal, Withdrawal};

// impl_crud!(IUserRepository, User, NewUser, users_table::table);
// impl_crud!(IProfileRepository, Profile, NewProfile, profiles::table);
// impl_crud!(IWithdrawalRepository, Withdrawal, NewWithdrawal, withdrawals::table);
// impl_crud!(IRefreshTokenRepository, RefreshToken, NewRefreshToken, refresh_tokens::table);
// impl_crud!(ICreatorSettingRepository, CreatorSetting, NewCreatorSetting, creator_settings::table);
// impl_crud!(IPostRepository, Post, NewPost, posts::table);
// impl_crud!(IPpvPurchaseRepository, PpvPurchase, NewPpvPurchase, ppv_purchases::table);
// impl_crud!(ITipRepository, Tip, NewTip, tips::table);
// impl_crud!(IReportRepository, Report, NewReport, reports::table);
// impl_crud!(IKycVerificationRepository, KycVerification, NewKycVerification, kyc_verifications::table);
// impl_crud!(IAuditLogRepository, AuditLog, NewAuditLog, audit_logs::table);
// impl_crud!(IUserRoleRepository, UserRole, NewUserRole, users_roles::table);

// -- Auth-specific implementations for BaseRepo

use crate::repositories::base::BaseRepo;
use uuid::Uuid;

#[async_trait::async_trait]
impl IUserRepository for BaseRepo {
    // Basic CRUD methods
    async fn all(&self) -> diesel::QueryResult<Vec<User>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| {
            users_table::table
                .select(User::as_select())
                .load::<User>(conn)
        })
        .await
    }

    async fn find(&self, uid: &Uuid) -> diesel::QueryResult<User> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        let uid_val = *uid;
        self.run(move |conn| {
            users_table::table
                .find(uid_val)
                .select(User::as_select())
                .first::<User>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewUser) -> diesel::QueryResult<User> {
        let item = item.clone();
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            diesel::insert_into(users_table::table)
                .values((
                    id.eq(item.id),
                    email.eq(&item.email),
                    encrypted_password.eq(&item.encrypted_password),
                    confirmation_token.eq(item.confirmation_token),
                    created_at.eq(item.created_at),
                    updated_at.eq(item.updated_at),
                ))
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, uid: &Uuid, item: &NewUser) -> diesel::QueryResult<User> {
        let item = item.clone();
        let uid = *uid;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            diesel::update(users_table::table.find(uid))
                .set((
                    email.eq(&item.email),
                    encrypted_password.eq(&item.encrypted_password),
                    updated_at.eq(item.updated_at),
                ))
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, uid: &Uuid) -> diesel::QueryResult<usize> {
        use diesel::{QueryDsl, RunQueryDsl};
        let uid_val = *uid;
        self.run(move |conn| diesel::delete(users_table::table.find(uid_val)).execute(conn))
            .await
    }

    // Auth-specific methods
    async fn find_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl::*;
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
        };
        let search = username_or_email.to_string();
        self.run(move |conn| {
            users
                .filter(email.eq(search.as_str()))
                .select(User::as_select())
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn find_by_email(&self, email_addr: &str) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl::*;
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
        };
        let search = email_addr.to_string();
        self.run(move |conn| {
            users
                .filter(email.eq(search.as_str()))
                .select(User::as_select())
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn find_by_reset_token(&self, token: &str) -> diesel::QueryResult<Option<User>> {
        use crate::db::schema::users::dsl::*;
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
        };
        let search = token.to_string();
        self.run(move |conn| {
            users
                .filter(reset_password_token.eq(&search))
                .select(User::as_select())
                .first::<User>(conn)
                .optional()
        })
        .await
    }

    async fn update_login_info(
        &self,
        user_id: &Uuid,
        curr_sign_in_at: Option<chrono::NaiveDateTime>,
        last_sign_in_at_opt: Option<chrono::NaiveDateTime>,
        curr_sign_in_ip: Option<ipnet::IpNet>,
        last_sign_in_ip_opt: Option<ipnet::IpNet>,
    ) -> diesel::QueryResult<User> {
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    current_sign_in_at.eq(curr_sign_in_at),
                    last_sign_in_at.eq(last_sign_in_at_opt),
                    current_sign_in_ip.eq(curr_sign_in_ip),
                    last_sign_in_ip.eq(last_sign_in_ip_opt),
                    sign_in_count.eq(sign_in_count + 1),
                ))
                .returning(User::as_returning())
                .get_result::<User>(conn)
        })
        .await
    }

    async fn update_password(
        &self,
        user_id: &Uuid,
        new_encrypted_password: &str,
    ) -> diesel::QueryResult<usize> {
        let pwd = new_encrypted_password.to_string();
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    encrypted_password.eq(pwd),
                    reset_password_token.eq::<Option<String>>(None),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn update_reset_token(
        &self,
        user_id: &Uuid,
        token: Option<String>,
        sent_at: Option<chrono::NaiveDateTime>,
    ) -> diesel::QueryResult<usize> {
        let tok = token;
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    reset_password_token.eq(tok),
                    reset_password_sent_at.eq(sent_at),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn confirm_email(&self, token: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let tok = token.to_string();
        self.run(move |conn| {
            diesel::update(users.filter(confirmation_token.eq(&tok)))
                .set((
                    confirmed_at.eq(Some(chrono::Utc::now().naive_utc())),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn record_failed_login(
        &self,
        user_id: &Uuid,
        max_attempts: i32,
    ) -> diesel::QueryResult<usize> {
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    failed_attempts.eq(failed_attempts + 1),
                    locked_at.eq(diesel::dsl::case_when(
                        failed_attempts.ge(max_attempts - 1),
                        Some(chrono::Utc::now().naive_utc()),
                    )
                    .otherwise(None::<chrono::NaiveDateTime>)),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn record_successful_login(
        &self,
        user_id: &Uuid,
        ip: Option<ipnet::IpNet>,
    ) -> diesel::QueryResult<usize> {
        let ip_clone = ip.clone();
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    sign_in_count.eq(sign_in_count + 1),
                    current_sign_in_at.eq(Some(chrono::Utc::now().naive_utc())),
                    last_sign_in_at.eq(current_sign_in_at),
                    current_sign_in_ip.eq(ip_clone.clone()),
                    last_sign_in_ip.eq(current_sign_in_ip),
                    failed_attempts.eq(0),
                    locked_at.eq::<Option<chrono::NaiveDateTime>>(None),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn get_user_roles(&self, user_id: &Uuid) -> diesel::QueryResult<Vec<String>> {
        let user_id = *user_id;
        use crate::db::schema::{roles, users_roles};
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            users_roles::table
                .inner_join(roles::table.on(users_roles::role_id.eq(roles::id)))
                .filter(users_roles::user_id.eq(user_id))
                .select(roles::name)
                .load::<String>(conn)
        })
        .await
    }

    async fn create_password_reset_token(
        &self,
        user_id: &Uuid,
        token: &str,
        sent_at: chrono::NaiveDateTime,
    ) -> diesel::QueryResult<usize> {
        let tok = token.to_string();
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    reset_password_token.eq(Some(tok)),
                    reset_password_sent_at.eq(Some(sent_at)),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> diesel::QueryResult<usize> {
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let tok = token.to_string();
        let pwd = new_password.to_string();
        self.run(move |conn| {
            diesel::update(users.filter(reset_password_token.eq(&tok)))
                .set((
                    encrypted_password.eq(pwd),
                    reset_password_token.eq::<Option<String>>(None),
                    reset_password_sent_at.eq::<Option<chrono::NaiveDateTime>>(None),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn set_otp_secret(&self, user_id: &Uuid, secret: &str) -> diesel::QueryResult<usize> {
        let sec = secret.to_string();
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    otp_secret.eq(Some(sec)),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn enable_2fa(
        &self,
        user_id: &Uuid,
        backup_codes: &[String],
    ) -> diesel::QueryResult<usize> {
        let codes = backup_codes.to_vec();
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    otp_enabled_at.eq(Some(chrono::Utc::now().naive_utc())),
                    otp_backup_codes.eq(Some(codes)),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }

    async fn disable_2fa(&self, user_id: &Uuid) -> diesel::QueryResult<usize> {
        let user_id = *user_id;
        use crate::db::schema::users::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(users.find(user_id))
                .set((
                    otp_secret.eq::<Option<String>>(None),
                    otp_enabled_at.eq::<Option<chrono::NaiveDateTime>>(None),
                    otp_backup_codes.eq::<Option<Vec<String>>>(None),
                    updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(conn)
        })
        .await
    }
}

// -- IProfileRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IProfileRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Profile>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| {
            profiles_table::table
                .select(Profile::as_select())
                .load::<Profile>(conn)
        })
        .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<Profile> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            profiles_table::table
                .find(pid)
                .select(Profile::as_select())
                .first::<Profile>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewProfile) -> diesel::QueryResult<Profile> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(profiles_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, pid: &Uuid, item: &NewProfile) -> diesel::QueryResult<Profile> {
        let item = item.clone();
        let pid = *pid;
        use crate::db::schema::profiles::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            diesel::update(profiles_table::table.find(pid))
                .set((
                    first_name.eq(item.first_name),
                    last_name.eq(item.last_name),
                    display_name.eq(item.display_name),
                    slug.eq(item.slug),
                    bio.eq(item.bio),
                    birthday.eq(item.birthday),
                    avatar_url.eq(item.avatar_url),
                    cover_url.eq(item.cover_url),
                    social_network.eq(item.social_network),
                    is_creator.eq(item.is_creator),
                    is_agency.eq(item.is_agency),
                    status.eq(item.status),
                ))
                .returning(Profile::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(profiles_table::table.find(pid)).execute(conn))
            .await
    }

    async fn find_by_user_id(&self, uid: &Uuid) -> diesel::QueryResult<Option<Profile>> {
        let uid = *uid;
        use crate::db::schema::profiles::dsl::*;
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
        };
        self.run(move |conn| {
            profiles_table::table
                .filter(user_id.eq(uid))
                .select(Profile::as_select())
                .first::<Profile>(conn)
                .optional()
        })
        .await
    }
}

// -- IWithdrawalRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IWithdrawalRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Withdrawal>> {
        use diesel::RunQueryDsl;
        self.run(|conn| withdrawals_table::table.load::<Withdrawal>(conn))
            .await
    }

    async fn find(&self, wid: &Uuid) -> diesel::QueryResult<Withdrawal> {
        let wid = *wid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| withdrawals_table::table.find(wid).first::<Withdrawal>(conn))
            .await
    }

    async fn create(&self, item: &NewWithdrawal) -> diesel::QueryResult<Withdrawal> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(withdrawals_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, wid: &Uuid, item: &NewWithdrawal) -> diesel::QueryResult<Withdrawal> {
        use diesel::{QueryDsl, RunQueryDsl};
        let item = item.clone();
        let wid = *wid;
        self.run(move |conn| {
            diesel::update(withdrawals_table::table.find(wid))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, wid: &Uuid) -> diesel::QueryResult<usize> {
        let wid = *wid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(withdrawals_table::table.find(wid)).execute(conn))
            .await
    }
}

// -- IRefreshTokenRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IRefreshTokenRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<RefreshToken>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| {
            refresh_tokens_table::table
                .select(RefreshToken::as_select())
                .load::<RefreshToken>(conn)
        })
        .await
    }

    async fn find(&self, tid: &Uuid) -> diesel::QueryResult<RefreshToken> {
        let tid = *tid;
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            refresh_tokens_table::table
                .find(tid)
                .select(RefreshToken::as_select())
                .first::<RefreshToken>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewRefreshToken) -> diesel::QueryResult<RefreshToken> {
        use diesel::{RunQueryDsl, SelectableHelper};
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(refresh_tokens_table::table)
                .values(&item)
                .returning(RefreshToken::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, tid: &Uuid) -> diesel::QueryResult<usize> {
        let tid = *tid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(refresh_tokens_table::table.find(tid)).execute(conn))
            .await
    }

    async fn find_by_token_hash(
        &self,
        token_hash_str: &str,
    ) -> diesel::QueryResult<Option<RefreshToken>> {
        let hash = token_hash_str.to_string();
        use crate::db::schema::refresh_tokens::dsl::*;
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
        };
        self.run(move |conn| {
            refresh_tokens_table::table
                .filter(token_hash.eq(hash))
                .select(RefreshToken::as_select())
                .first::<RefreshToken>(conn)
                .optional()
        })
        .await
    }

    async fn revoke(&self, tid: &Uuid) -> diesel::QueryResult<usize> {
        let tid = *tid;
        use crate::db::schema::refresh_tokens::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(refresh_tokens_table::table.find(tid))
                .set(revoked_at.eq(Some(chrono::Utc::now())))
                .execute(conn)
        })
        .await
    }

    async fn revoke_all_for_user(&self, uid: &Uuid) -> diesel::QueryResult<usize> {
        let uid = *uid;
        use crate::db::schema::refresh_tokens::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(refresh_tokens_table::table.filter(user_id.eq(uid)))
                .set(revoked_at.eq(Some(chrono::Utc::now())))
                .execute(conn)
        })
        .await
    }
}

// -- ICreatorSettingRepository implementation for BaseRepo

#[async_trait::async_trait]
impl ICreatorSettingRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<CreatorSetting>> {
        use diesel::RunQueryDsl;
        self.run(|conn| creator_settings_table::table.load::<CreatorSetting>(conn))
            .await
    }

    async fn find(&self, csid: &Uuid) -> diesel::QueryResult<CreatorSetting> {
        let csid = *csid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            creator_settings_table::table
                .find(csid)
                .first::<CreatorSetting>(conn)
        })
        .await
    }

    async fn find_by_profile(&self, prof_id: &Uuid) -> diesel::QueryResult<Option<CreatorSetting>> {
        let prof_id = *prof_id;
        use crate::db::schema::creator_settings::dsl::*;
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            creator_settings_table::table
                .filter(profile_id.eq(prof_id))
                .first::<CreatorSetting>(conn)
                .optional()
        })
        .await
    }

    async fn create(&self, item: &NewCreatorSetting) -> diesel::QueryResult<CreatorSetting> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(creator_settings_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(
        &self,
        csid: &Uuid,
        item: &NewCreatorSetting,
    ) -> diesel::QueryResult<CreatorSetting> {
        let item = item.clone();
        let csid = *csid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(creator_settings_table::table.find(csid))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, csid: &Uuid) -> diesel::QueryResult<usize> {
        let csid = *csid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(creator_settings_table::table.find(csid)).execute(conn))
            .await
    }
}

// -- IPostRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IPostRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Post>> {
        use diesel::RunQueryDsl;
        self.run(|conn| posts_table::table.load::<Post>(conn)).await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<Post> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| posts_table::table.find(pid).first::<Post>(conn))
            .await
    }

    async fn find_by_creator(&self, creator_id: &Uuid) -> diesel::QueryResult<Vec<Post>> {
        let creator_id = *creator_id;
        use crate::db::schema::posts::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            posts_table::table
                .filter(creator_profile_id.eq(creator_id))
                .load::<Post>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewPost) -> diesel::QueryResult<Post> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(posts_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, pid: &Uuid, item: &NewPost) -> diesel::QueryResult<Post> {
        let item = item.clone();
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(posts_table::table.find(pid))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(posts_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- IPpvPurchaseRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IPpvPurchaseRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<PpvPurchase>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| {
            ppv_purchases_table::table
                .select(PpvPurchase::as_select())
                .load::<PpvPurchase>(conn)
        })
        .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<PpvPurchase> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            ppv_purchases_table::table
                .find(pid)
                .select(PpvPurchase::as_select())
                .first::<PpvPurchase>(conn)
        })
        .await
    }

    async fn find_by_buyer(&self, buyer_id: &Uuid) -> diesel::QueryResult<Vec<PpvPurchase>> {
        let buyer_id = *buyer_id;
        use crate::db::schema::ppv_purchases::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            ppv_purchases_table::table
                .filter(buyer_profile_id.eq(buyer_id))
                .select(PpvPurchase::as_select())
                .load::<PpvPurchase>(conn)
        })
        .await
    }

    async fn find_by_post(&self, post: &Uuid) -> diesel::QueryResult<Vec<PpvPurchase>> {
        let post = *post;
        use crate::db::schema::ppv_purchases::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            ppv_purchases_table::table
                .filter(post_id.eq(post))
                .select(PpvPurchase::as_select())
                .load::<PpvPurchase>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewPpvPurchase) -> diesel::QueryResult<PpvPurchase> {
        use diesel::{RunQueryDsl, SelectableHelper};
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(ppv_purchases_table::table)
                .values(&item)
                .returning(PpvPurchase::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(ppv_purchases_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- ITipRepository implementation for BaseRepo

#[async_trait::async_trait]
impl ITipRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Tip>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| tips_table::table.select(Tip::as_select()).load::<Tip>(conn))
            .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<Tip> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            tips_table::table
                .find(pid)
                .select(Tip::as_select())
                .first::<Tip>(conn)
        })
        .await
    }

    async fn find_by_sender(&self, sender_id: &Uuid) -> diesel::QueryResult<Vec<Tip>> {
        let sender_id = *sender_id;
        use crate::db::schema::tips::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            tips_table::table
                .filter(sender_profile_id.eq(sender_id))
                .select(Tip::as_select())
                .load::<Tip>(conn)
        })
        .await
    }

    async fn find_by_receiver(&self, receiver_id: &Uuid) -> diesel::QueryResult<Vec<Tip>> {
        let receiver_id = *receiver_id;
        use crate::db::schema::tips::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            tips_table::table
                .filter(receiver_profile_id.eq(receiver_id))
                .select(Tip::as_select())
                .load::<Tip>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewTip) -> diesel::QueryResult<Tip> {
        use diesel::{RunQueryDsl, SelectableHelper};
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(tips_table::table)
                .values(&item)
                .returning(Tip::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(tips_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- IReportRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IReportRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<Report>> {
        use diesel::RunQueryDsl;
        self.run(|conn| reports_table::table.load::<Report>(conn))
            .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<Report> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| reports_table::table.find(pid).first::<Report>(conn))
            .await
    }

    async fn find_by_status(&self, status_filter: i32) -> diesel::QueryResult<Vec<Report>> {
        use crate::db::schema::reports::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            reports_table::table
                .filter(status.eq(status_filter))
                .load::<Report>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewReport) -> diesel::QueryResult<Report> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(reports_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(&self, pid: &Uuid, item: &NewReport) -> diesel::QueryResult<Report> {
        let item = item.clone();
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(reports_table::table.find(pid))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(reports_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- IKycVerificationRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IKycVerificationRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<KycVerification>> {
        use diesel::RunQueryDsl;
        self.run(|conn| kyc_verifications_table::table.load::<KycVerification>(conn))
            .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<KycVerification> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            kyc_verifications_table::table
                .find(pid)
                .first::<KycVerification>(conn)
        })
        .await
    }

    async fn find_by_profile(
        &self,
        profile: &Uuid,
    ) -> diesel::QueryResult<Option<KycVerification>> {
        let profile = *profile;
        use crate::db::schema::kyc_verifications::dsl::*;
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            kyc_verifications_table::table
                .filter(profile_id.eq(profile))
                .first::<KycVerification>(conn)
                .optional()
        })
        .await
    }

    async fn find_by_status(
        &self,
        status_filter: i32,
    ) -> diesel::QueryResult<Vec<KycVerification>> {
        use crate::db::schema::kyc_verifications::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            kyc_verifications_table::table
                .filter(status.eq(status_filter))
                .load::<KycVerification>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewKycVerification) -> diesel::QueryResult<KycVerification> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(kyc_verifications_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn update(
        &self,
        pid: &Uuid,
        item: &NewKycVerification,
    ) -> diesel::QueryResult<KycVerification> {
        let item = item.clone();
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::update(kyc_verifications_table::table.find(pid))
                .set(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(kyc_verifications_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- IAuditLogRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IAuditLogRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<AuditLog>> {
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(|conn| {
            audit_logs_table::table
                .select(AuditLog::as_select())
                .load::<AuditLog>(conn)
        })
        .await
    }

    async fn find(&self, pid: &Uuid) -> diesel::QueryResult<AuditLog> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            audit_logs_table::table
                .find(pid)
                .select(AuditLog::as_select())
                .first::<AuditLog>(conn)
        })
        .await
    }

    async fn find_by_user(&self, uid: &Uuid) -> diesel::QueryResult<Vec<AuditLog>> {
        let uid = *uid;
        use crate::db::schema::audit_logs::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            audit_logs_table::table
                .filter(user_id.eq(uid))
                .order(created_at.desc())
                .select(AuditLog::as_select())
                .load::<AuditLog>(conn)
        })
        .await
    }

    async fn find_by_action(&self, action_name: &str) -> diesel::QueryResult<Vec<AuditLog>> {
        let act = action_name.to_string();
        use crate::db::schema::audit_logs::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
        self.run(move |conn| {
            audit_logs_table::table
                .filter(action.eq(&act))
                .order(created_at.desc())
                .select(AuditLog::as_select())
                .load::<AuditLog>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewAuditLog) -> diesel::QueryResult<AuditLog> {
        use diesel::{RunQueryDsl, SelectableHelper};
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(audit_logs_table::table)
                .values(&item)
                .returning(AuditLog::as_returning())
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, pid: &Uuid) -> diesel::QueryResult<usize> {
        let pid = *pid;
        use diesel::{QueryDsl, RunQueryDsl};
        self.run(move |conn| diesel::delete(audit_logs_table::table.find(pid)).execute(conn))
            .await
    }
}

// -- IUserRoleRepository implementation for BaseRepo

#[async_trait::async_trait]
impl IUserRoleRepository for BaseRepo {
    async fn all(&self) -> diesel::QueryResult<Vec<UserRole>> {
        use diesel::RunQueryDsl;
        self.run(|conn| users_roles_table::table.load::<UserRole>(conn))
            .await
    }

    async fn find(&self, user: &Uuid, role: &Uuid) -> diesel::QueryResult<UserRole> {
        let user = *user;
        let role = *role;
        use crate::db::schema::users_roles::dsl::*;
        use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            users_roles_table::table
                .filter(user_id.eq(user).and(role_id.eq(role)))
                .first::<UserRole>(conn)
        })
        .await
    }

    async fn find_by_user(&self, uid: &Uuid) -> diesel::QueryResult<Vec<UserRole>> {
        let uid = *uid;
        use crate::db::schema::users_roles::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            users_roles_table::table
                .filter(user_id.eq(uid))
                .load::<UserRole>(conn)
        })
        .await
    }

    async fn find_by_role(&self, rid: &Uuid) -> diesel::QueryResult<Vec<UserRole>> {
        let rid = *rid;
        use crate::db::schema::users_roles::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            users_roles_table::table
                .filter(role_id.eq(rid))
                .load::<UserRole>(conn)
        })
        .await
    }

    async fn create(&self, item: &NewUserRole) -> diesel::QueryResult<UserRole> {
        use diesel::RunQueryDsl;
        let item = item.clone();
        self.run(move |conn| {
            diesel::insert_into(users_roles_table::table)
                .values(&item)
                .get_result(conn)
        })
        .await
    }

    async fn destroy(&self, uid: &Uuid, rid: &Uuid) -> diesel::QueryResult<usize> {
        let uid = *uid;
        let rid = *rid;
        use crate::db::schema::users_roles::dsl::*;
        use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
        self.run(move |conn| {
            diesel::delete(users_roles_table::table.filter(user_id.eq(uid).and(role_id.eq(rid))))
                .execute(conn)
        })
        .await
    }
}
pub mod test_utils;
