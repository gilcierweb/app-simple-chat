#![allow(dead_code)]

use crate::{errors::ApiError, middleware::auth_middleware::extract_claims};
use actix_web::HttpRequest;

/// Guard: ensure authenticated user has a specific role.
/// Usage in handlers: `require_role(&req, "creator")?;`
pub fn require_role(req: &HttpRequest, role: &str) -> Result<(), ApiError> {
    let claims = extract_claims(req)?;
    if claims.has_role(role) || claims.is_admin() {
        Ok(())
    } else {
        Err(ApiError::Forbidden(format!(
            "Role '{}' required for this action",
            role
        )))
    }
}

/// Guard: ensure the authenticated user IS the resource owner or an admin.
pub fn require_owner_or_admin(
    req: &HttpRequest,
    owner_profile_id: uuid::Uuid,
) -> Result<(), ApiError> {
    let claims = extract_claims(req)?;
    let requester = claims
        .profile_id()
        .ok_or(ApiError::Forbidden("Profile ID not found".to_string()))?;
    if requester == owner_profile_id || claims.is_admin() {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            "You don't have permission to access this resource".to_string(),
        ))
    }
}
