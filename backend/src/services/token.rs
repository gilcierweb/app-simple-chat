use crate::config::AppConfig;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Option<Vec<String>>,
    pub profile_id: Option<String>,
}

impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles
            .as_ref()
            .map(|r| r.contains(&role.to_string()))
            .unwrap_or(false)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    pub fn profile_id(&self) -> Option<Uuid> {
        self.profile_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok())
    }
}

pub fn verify_access_token(
    token: &str,
    config: &AppConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
