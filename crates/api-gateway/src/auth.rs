//! Centralized Authentication for API Gateway

use actix_web::{Error as ActixError, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// JWT Claims with extended information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// Authentication context extracted from request
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl AuthContext {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            roles: vec![],
            permissions: vec![],
        }
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles;
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
}

/// Validate JWT token from request
pub fn validate_token(req: &HttpRequest) -> Result<AuthContext, ActixError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "bssoss-secret".to_string());

    let header_value = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = header_value
        .to_str()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid authorization header"))?
        .replace("Bearer ", "");

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    let claims = token_data.claims;
    Ok(AuthContext {
        user_id: claims.sub,
        roles: claims.roles.unwrap_or_default(),
        permissions: claims.permissions.unwrap_or_default(),
    })
}

/// Extract authentication context from request (optional)
pub fn extract_auth_context(req: &HttpRequest) -> Option<AuthContext> {
    validate_token(req).ok()
}
