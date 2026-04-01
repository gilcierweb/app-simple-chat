use crate::api_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct WsState;

pub use crate::controllers::{
    auth_controller, conversations_controller, health_controller, keys_controller,
    messages_controller, profiles_controller, users_controller, ws_controller,
};

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig, redis_pool: deadpool_redis::Pool) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    )
    .service(
        web::scope("/api/v1")
            .wrap(crate::middleware::rate_limit_middleware::RateLimiter::new(
                redis_pool.clone(),
                crate::middleware::rate_limit_middleware::RATE_API,
            ))
            // Auth routes
            .service(
                web::scope("/auth")
                    .wrap(crate::middleware::rate_limit_middleware::RateLimiter::new(
                        redis_pool.clone(),
                        crate::middleware::rate_limit_middleware::RATE_AUTH,
                    ))
                    .service(auth_controller::login)
                    .service(auth_controller::register)
                    .service(auth_controller::confirm)
                    .service(auth_controller::me)
                    .service(auth_controller::refresh)
                    .service(auth_controller::logout)
                    .service(auth_controller::recover_password)
                    .service(auth_controller::reset_password)
                    .service(auth_controller::setup_2fa)
                    .service(auth_controller::enable_2fa)
                    .service(auth_controller::disable_2fa)
                    .service(auth_controller::change_password),
            )
            // User routes
            .service(users_controller::get_me)
            .service(users_controller::update_me)
            .service(users_controller::list_sessions)
            .service(users_controller::revoke_session)
            // User lookup
            .service(conversations_controller::lookup_user_by_email)
            // Profile routes
            .service(profiles_controller::get_profile)
            .service(profiles_controller::update_profile)
            // Health check
            .service(health_controller::health_check)
            // Conversations routes
            .service(conversations_controller::list_conversations)
            .service(conversations_controller::create_conversation)
            .service(conversations_controller::get_conversation_messages)
            .service(conversations_controller::send_message)
            .service(conversations_controller::delete_message)
            .service(conversations_controller::lookup_user_by_email)
            // Messages routes
            .service(messages_controller::list)
            .service(messages_controller::send)
            .service(messages_controller::delete)
            .service(messages_controller::update_receipt)
            // Keys routes
            .service(keys_controller::upload_keys)
            .service(keys_controller::get_prekey_bundle)
            // WebSocket token endpoint
            .service(ws_controller::get_ws_token)
            // WebSocket route (inside /api/v1 scope)
            .service(web::resource("/ws").route(web::get().to(crate::ws::server::ws_handler))),
    );
}
