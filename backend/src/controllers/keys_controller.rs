use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::errors::AppResult;
use crate::repositories::AppContainer;
use crate::services::e2e_keys_service::{E2eKeysService, UploadKeysRequest};

#[post("/keys/upload")]
pub async fn upload_keys(
    container: web::Data<AppContainer>,
    user: AuthUser,
    req: web::Json<UploadKeysRequest>,
) -> AppResult<HttpResponse> {
    E2eKeysService::upload_keys(&container.into_inner(), user.claims().sub, req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Keys uploaded successfully"
    })))
}

#[get("/keys/{user_id}")]
pub async fn get_prekey_bundle(
    container: web::Data<AppContainer>,
    _user: AuthUser, // Must be authenticated to fetch peer keys
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let target_user_id = path.into_inner();
    
    let bundle = E2eKeysService::fetch_bundle(&container.into_inner(), target_user_id).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": bundle
    })))
}
