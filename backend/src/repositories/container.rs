use std::sync::Arc;

use crate::config::app_config::AppConfig;
use crate::db::database::DBPool;
use crate::repositories::base::BaseRepo;
use crate::repositories::users_repository::IUserRepository;
use crate::repositories::profiles_repository::IProfileRepository;
use crate::repositories::refresh_tokens_repository::IRefreshTokenRepository;
use crate::repositories::audit_logs_repository::IAuditLogRepository;

pub struct AppContainer {
    pub config: Arc<AppConfig>,
    pub users: Arc<dyn IUserRepository>,
    pub profiles: Arc<dyn IProfileRepository>,
    pub refresh_tokens: Arc<dyn IRefreshTokenRepository>,
    pub audit_logs: Arc<dyn IAuditLogRepository>,
}

impl AppContainer {
    pub fn new(pool: DBPool, config: Arc<AppConfig>) -> Self {
        let base = BaseRepo::new(pool);
        Self {
            config,
            users: Arc::new(base.clone()),
            profiles: Arc::new(base.clone()),
            refresh_tokens: Arc::new(base.clone()),
            audit_logs: Arc::new(base),
        }
    }
}
