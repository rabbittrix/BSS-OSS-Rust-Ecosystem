//! Authentication utilities for TMF633

use actix_web::{Error as ActixError, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Validate JWT token from request
pub fn validate_token(req: &HttpRequest) -> Result<String, ActixError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "bssoss-secret".to_string());

    if let Some(header_value) = req.headers().get("Authorization") {
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

        Ok(token_data.claims.sub)
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Missing authorization header",
        ))
    }
}
