#![allow(dead_code)]

use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::{errors::AppError, middleware::auth::AuthUser, repositories::container::AppContainer};

#[derive(serde::Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub status: bool,
    pub social_network: serde_json::Value,
}

#[derive(serde::Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// GET /api/v1/profile
#[actix_web::get("/profile")]
pub async fn get_profile(
    user: AuthUser,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;

    let profile = container
        .profiles
        .find_by_user_id(&user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Profile not found".to_string()))?;

    Ok(HttpResponse::Ok().json(ProfileResponse {
        id: profile.id,
        user_id: profile.user_id,
        nickname: profile.nickname,
        bio: profile.bio,
        avatar_url: profile.avatar_url,
        status: profile.status,
        social_network: profile.social_network,
    }))
}

/// PATCH /api/v1/profile
#[actix_web::patch("/profile")]
pub async fn update_profile(
    user: AuthUser,
    body: web::Json<UpdateProfileRequest>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;

    let new_profile = crate::models::profile::NewProfile {
        user_id,
        first_name_enc: None,
        last_name_enc: None,
        phone_enc: None,
        nickname: body.nickname.clone(),
        bio: body.bio.clone(),
        birthday: None,
        avatar_url: body.avatar_url.clone(),
        status: true,
        social_network: serde_json::json!({}),
    };

    let _ = container
        .profiles
        .update(&user_id, &new_profile)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Profile updated"})))
}
