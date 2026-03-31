use actix_web::{HttpResponse, delete, get, patch, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::profile::{NewProfile, Profile},
    models::refresh_token::RefreshToken,
    repositories::container::AppContainer,
};

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub slug: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserMeResponse {
    pub user: UserInfo,
    pub profile: Profile,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

/// GET /api/users/me
#[get("/users/me")]
pub async fn get_me(user: AuthUser, container: web::Data<AppContainer>) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let user_data = container
        .users
        .find(&user_id)
        .await
        .map_err(|_| AppError::NotFound("User".to_string()))?;

    let profile = container
        .profiles
        .find_by_user_id(&user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Profile".to_string()))?;

    let roles = container
        .users
        .get_user_roles(&user_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(UserMeResponse {
        user: UserInfo {
            id: user_data.id,
            email: user_data.email,
        },
        profile,
        roles,
    }))
}

/// PATCH /api/users/me
#[patch("/users/me")]
pub async fn update_me(
    user: AuthUser,
    container: web::Data<AppContainer>,
    body: web::Json<UpdateProfileRequest>,
) -> AppResult<HttpResponse> {
    let profile_id = user.claims().profile_id;

    let profile = container
        .profiles
        .find(&profile_id)
        .await
        .map_err(|_| AppError::NotFound("Profile".to_string()))?;

    // Validate slug uniqueness if changed
    if let Some(ref slug) = body.slug {
        let all_profiles = container.profiles.all().await.map_err(AppError::Database)?;

        let taken = all_profiles
            .iter()
            .any(|p| p.slug.as_ref() == Some(slug) && p.id != profile_id);
        if taken {
            return Err(AppError::Conflict("This slug is already taken".to_string()));
        }
    }

    let updated_profile = NewProfile {
        user_id: profile.user_id,
        first_name: profile.first_name.clone(),
        last_name: profile.last_name.clone(),
        display_name: body.display_name.clone().or(profile.display_name.clone()),
        slug: body.slug.clone().or(profile.slug.clone()),
        bio: body.bio.clone().or(profile.bio.clone()),
        birthday: profile.birthday,
        avatar_url: body.avatar_url.clone().or(profile.avatar_url.clone()),
        cover_url: body.cover_url.clone().or(profile.cover_url.clone()),
        social_network: profile.social_network.clone(),
        is_creator: profile.is_creator,
        is_agency: profile.is_agency,
        status: profile.status,
    };

    let updated = container
        .profiles
        .update(&profile_id, &updated_profile)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(updated))
}

/// GET /api/users/me/sessions
#[get("/users/me/sessions")]
pub async fn list_sessions(
    user: AuthUser,
    container: web::Data<AppContainer>,
    pagination: web::Query<crate::utils::PaginationParams>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let all_tokens = container
        .refresh_tokens
        .all()
        .await
        .map_err(AppError::Database)?;

    let sessions: Vec<&RefreshToken> = all_tokens
        .iter()
        .filter(|t| t.user_id == user_id && t.revoked_at.is_none() && t.expires_at > Utc::now())
        .collect();

    let total = sessions.len() as i64;
    let response =
        crate::utils::PaginatedResponse::new(sessions, total, pagination.page, pagination.per_page);

    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/users/me/sessions/{id}
#[delete("/users/me/sessions/{id}")]
pub async fn revoke_session(
    user: AuthUser,
    container: web::Data<AppContainer>,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let session_id = path.into_inner();

    // Verify the session belongs to the user
    let token = container
        .refresh_tokens
        .find(&session_id)
        .await
        .map_err(|_| AppError::NotFound("Session".to_string()))?;

    if token.user_id != user_id {
        return Err(AppError::Forbidden("Not authorized".to_string()));
    }

    container
        .refresh_tokens
        .revoke(&session_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "revoked": true })))
}
