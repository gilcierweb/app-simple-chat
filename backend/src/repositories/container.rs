use std::sync::Arc;

use crate::config::app_config::AppConfig;
use crate::db::database::DBPool;
use crate::repositories::audit_logs_repository::IAuditLogRepository;
use crate::repositories::base::BaseRepo;
use crate::repositories::conversations_repository::{
    IConversationMemberRepository, IConversationRepository, IMessageRepository,
};
use crate::repositories::profiles_repository::IProfileRepository;
use crate::repositories::refresh_tokens_repository::IRefreshTokenRepository;
use crate::repositories::users_repository::IUserRepository;
use crate::ws::server::WsState;
use diesel::QueryResult;

pub struct AppContainer {
    pub config: Arc<AppConfig>,
    pub users: Arc<dyn IUserRepository>,
    pub profiles: Arc<dyn IProfileRepository>,
    pub refresh_tokens: Arc<dyn IRefreshTokenRepository>,
    pub audit_logs: Arc<dyn IAuditLogRepository>,
    pub conversations: Arc<dyn IConversationRepository>,
    pub conversation_members: Arc<dyn IConversationMemberRepository>,
    pub messages: Arc<dyn IMessageRepository>,
    pub ws_state: Arc<WsState>,
    base: BaseRepo,
}

impl AppContainer {
    pub fn new(pool: DBPool, config: Arc<AppConfig>, ws_state: Arc<WsState>) -> Self {
        let base = BaseRepo::new(pool);
        Self {
            config,
            users: Arc::new(base.clone()),
            profiles: Arc::new(base.clone()),
            refresh_tokens: Arc::new(base.clone()),
            audit_logs: Arc::new(base.clone()),
            conversations: Arc::new(base.clone()),
            conversation_members: Arc::new(base.clone()),
            messages: Arc::new(base.clone()),
            ws_state,
            base,
        }
    }

    /// Run a database query inline
    pub async fn run<F, T>(&self, f: F) -> QueryResult<T>
    where
        F: FnOnce(&mut diesel::PgConnection) -> QueryResult<T> + Send + 'static,
        T: Send + 'static,
    {
        self.base.run(f).await
    }
}
