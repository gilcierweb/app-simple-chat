#![allow(dead_code)]

use std::sync::Arc;

use crate::config::app_config::AppConfig;
use crate::db::database::DBPool;
use crate::repositories::audit_logs_repository::IAuditLogRepository;
use crate::repositories::base::BaseRepo;
use crate::repositories::creator_settings_repository::ICreatorSettingRepository;
use crate::repositories::kyc_verifications_repository::IKycVerificationRepository;
use crate::repositories::live_streams_impl::LiveStreamsRepository;
use crate::repositories::live_streams_repository::ILiveStreamRepository;
use crate::repositories::messages_impl::MessagesRepository;
use crate::repositories::messages_repository::IMessageRepository;
use crate::repositories::notification_preferences_impl::NotificationPreferencesRepository;
use crate::repositories::notification_preferences_repository::INotificationPreferenceRepository;
use crate::repositories::notifications_impl::NotificationsRepository;
use crate::repositories::notifications_repository::INotificationRepository;
use crate::repositories::posts_repository::IPostRepository;
use crate::repositories::ppv_purchases_repository::IPpvPurchaseRepository;
use crate::repositories::profiles_repository::IProfileRepository;
use crate::repositories::refresh_tokens_repository::IRefreshTokenRepository;
use crate::repositories::reports_repository::IReportRepository;
use crate::repositories::tips_repository::ITipRepository;
use crate::repositories::user_roles_repository::IUserRoleRepository;
use crate::repositories::users_repository::IUserRepository;
use crate::repositories::withdrawals_repository::IWithdrawalRepository;

/// Central dependency injection container.
/// Groups all repositories behind trait objects (`Arc<dyn Trait>`) for:
/// - Thread-safe sharing across Actix workers
/// - Easy swapping with mock implementations in tests
pub struct AppContainer {
    pub config: Arc<AppConfig>,
    pub users: Arc<dyn IUserRepository>,
    pub profiles: Arc<dyn IProfileRepository>,
    pub withdrawals: Arc<dyn IWithdrawalRepository>,
    pub refresh_tokens: Arc<dyn IRefreshTokenRepository>,
    pub creator_settings: Arc<dyn ICreatorSettingRepository>,
    pub posts: Arc<dyn IPostRepository>,
    pub ppv_purchases: Arc<dyn IPpvPurchaseRepository>,
    pub tips: Arc<dyn ITipRepository>,
    pub reports: Arc<dyn IReportRepository>,
    pub kyc_verifications: Arc<dyn IKycVerificationRepository>,
    pub audit_logs: Arc<dyn IAuditLogRepository>,
    pub user_roles: Arc<dyn IUserRoleRepository>,
    pub messages: Arc<dyn IMessageRepository>,
    pub notification_preferences: Arc<dyn INotificationPreferenceRepository>,
    pub notifications: Arc<dyn INotificationRepository>,
    pub live_streams: Arc<dyn ILiveStreamRepository>,
    pub stripe: Arc<crate::services::subscription_service::StripeService>,
    pub cache: Arc<crate::services::cache_service::CacheManager>,
}

impl AppContainer {
    pub fn new(pool: DBPool, redis_pool: deadpool_redis::Pool, config: AppConfig) -> Self {
        let stripe = Arc::new(crate::services::subscription_service::StripeService::new(
            config.stripe_secret_key.clone(),
            config.stripe_webhook_secret.clone(),
        ));
        let cache = Arc::new(crate::services::cache_service::CacheManager::from_pool(
            redis_pool,
            std::time::Duration::from_secs(3600),
        ));

        Self {
            config: Arc::new(config),
            users: Arc::new(BaseRepo::new(pool.clone())),
            profiles: Arc::new(BaseRepo::new(pool.clone())),
            withdrawals: Arc::new(BaseRepo::new(pool.clone())),
            refresh_tokens: Arc::new(BaseRepo::new(pool.clone())),
            creator_settings: Arc::new(BaseRepo::new(pool.clone())),
            posts: Arc::new(BaseRepo::new(pool.clone())),
            ppv_purchases: Arc::new(BaseRepo::new(pool.clone())),
            tips: Arc::new(BaseRepo::new(pool.clone())),
            reports: Arc::new(BaseRepo::new(pool.clone())),
            kyc_verifications: Arc::new(BaseRepo::new(pool.clone())),
            audit_logs: Arc::new(BaseRepo::new(pool.clone())),
            user_roles: Arc::new(BaseRepo::new(pool.clone())),
            messages: Arc::new(MessagesRepository::new(pool.clone())),
            notification_preferences: Arc::new(NotificationPreferencesRepository::new(
                pool.clone(),
            )),
            notifications: Arc::new(NotificationsRepository::new(pool.clone())),
            live_streams: Arc::new(LiveStreamsRepository::new(pool.clone())),
            stripe,
            cache,
        }
    }
}
