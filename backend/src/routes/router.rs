use crate::api_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct WsState;

pub use crate::controllers::{
    auth_controller, health_controller, users_controller, ws_controller,
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
            // Health check
            .service(health_controller::health_check)           
            // WebSocket token endpoint
            .service(ws_controller::get_ws_token)
            // WebSocket route (inside /api/v1 scope)
            .service(web::resource("/ws").route(web::get().to(crate::ws::server::ws_handler))),
    );
}
