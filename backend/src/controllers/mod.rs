pub mod auth_controller;
pub mod creator_settings_controller;
pub mod creator_stats_controller;
pub mod creators_controller;
pub mod health_controller;
pub mod kyc_controller;
pub mod live_controller;
pub mod media_controller;
pub mod messages_controller;
pub mod notification_preferences_controller;
pub mod notifications_controller;
pub mod posts_controller;
pub mod subscriptions_controller;
pub mod users_controller;
pub mod webhooks_controller;
pub mod withdrawals_controller;
pub mod ws_controller;

use actix_web::{HttpResponse, Responder};
use diesel::QueryResult;
use serde::Serialize;
use rust_i18n::t;

pub fn handle_result<T: Serialize>(result: QueryResult<T>) -> impl Responder {
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}

pub fn handle_result_created<T: Serialize>(result: QueryResult<T>) -> impl Responder {
    match result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub fn handle_result_no_content(result: QueryResult<usize>) -> impl Responder {
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}