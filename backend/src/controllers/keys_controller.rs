#![allow(dead_code)]

use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::user_keys;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::user_key::{NewUserKey, UserKey};
use crate::repositories::container::AppContainer;

#[derive(Debug, serde::Deserialize)]
pub struct UploadKeysRequest {
    pub identity_key: String,
    pub signed_prekey: SignedPrekeyBody,
    pub one_time_prekeys: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SignedPrekeyBody {
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PrekeyBundle {
    pub user_id: Uuid,
    pub identity_key: String,
    pub signed_prekey: String,
    pub signed_prekey_signature: String,
    pub one_time_prekey: Option<String>,
    pub one_time_prekey_id: Option<Uuid>,
}

#[actix_web::post("/keys")]
pub async fn upload_keys(
    user: AuthUser,
    body: web::Json<UploadKeysRequest>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let user_id = user.claims().sub;
    let identity_key = body.identity_key.clone();
    let signed_prekey = body.signed_prekey.clone();
    let one_time_prekeys = body.one_time_prekeys.clone();

    container
        .run(move |conn| {
            use diesel::ExpressionMethods;
            let target_user_id = user_id;
            diesel::delete(user_keys::table)
                .filter(user_keys::user_id.eq(target_user_id))
                .filter(user_keys::key_type.ne(3i32))
                .execute(conn)
        })
        .await
        .map_err(AppError::Database)?;

    let identity = NewUserKey {
        id: Uuid::new_v4(),
        user_id,
        key_type: 1i32,
        public_key: identity_key,
        signature: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        expires_at: None,
        used_at: None,
    };
    container
        .run(move |conn| {
            diesel::insert_into(user_keys::table)
                .values(&identity)
                .execute(conn)
        })
        .await
        .map_err(AppError::Database)?;

    let spk = NewUserKey {
        id: Uuid::new_v4(),
        user_id,
        key_type: 2i32,
        public_key: signed_prekey.public_key,
        signature: Some(signed_prekey.signature),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        used_at: None,
    };
    container
        .run(move |conn| {
            diesel::insert_into(user_keys::table)
                .values(&spk)
                .execute(conn)
        })
        .await
        .map_err(AppError::Database)?;

    for otpk in one_time_prekeys {
        let key = NewUserKey {
            id: Uuid::new_v4(),
            user_id,
            key_type: 3i32,
            public_key: otpk,
            signature: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            expires_at: None,
            used_at: None,
        };
        container
            .run(move |conn| {
                diesel::insert_into(user_keys::table)
                    .values(&key)
                    .execute(conn)
            })
            .await
            .map_err(AppError::Database)?;
    }

    tracing::info!("Keys uploaded for user");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Keys uploaded successfully"
    })))
}

#[actix_web::get("/keys/{user_id}")]
pub async fn get_prekey_bundle(
    _user: AuthUser,
    path: web::Path<Uuid>,
    container: web::Data<AppContainer>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();

    let identity_key: UserKey = container
        .run(move |conn| {
            use diesel::{ExpressionMethods, OptionalExtension};
            user_keys::table
                .filter(user_keys::user_id.eq(target_user_id))
                .filter(user_keys::key_type.eq(1i32))
                .first(conn)
                .optional()
        })
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("User keys not found".to_string()))?;

    let signed_prekey: UserKey = container
        .run(move |conn| {
            use diesel::ExpressionMethods;
            user_keys::table
                .filter(user_keys::user_id.eq(target_user_id))
                .filter(user_keys::key_type.eq(2i32))
                .order(user_keys::created_at.desc())
                .first(conn)
        })
        .await
        .map_err(AppError::Database)?;

    let otpk: Option<UserKey> = container
        .run(move |conn| {
            use diesel::{ExpressionMethods, OptionalExtension};
            user_keys::table
                .filter(user_keys::user_id.eq(target_user_id))
                .filter(user_keys::key_type.eq(3i32))
                .filter(user_keys::used_at.is_null())
                .first(conn)
                .optional()
        })
        .await
        .map_err(AppError::Database)?;

    let (otpk_key, otpk_id) = if let Some(k) = otpk {
        let key_id = k.id;
        let key_public = k.public_key.clone();
        container
            .run(move |conn| {
                use diesel::ExpressionMethods;
                diesel::update(user_keys::table.find(key_id))
                    .set(user_keys::used_at.eq(Some(chrono::Utc::now())))
                    .execute(conn)
            })
            .await
            .map_err(AppError::Database)?;
        (Some(key_public), Some(k.id))
    } else {
        (None, None)
    };

    Ok(HttpResponse::Ok().json(PrekeyBundle {
        user_id: target_user_id,
        identity_key: identity_key.public_key,
        signed_prekey: signed_prekey.public_key,
        signed_prekey_signature: signed_prekey.signature.unwrap_or_default(),
        one_time_prekey: otpk_key,
        one_time_prekey_id: otpk_id,
    }))
}
