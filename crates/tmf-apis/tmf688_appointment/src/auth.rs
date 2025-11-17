//! JWT Authentication for TMF688 API

use actix_web::{Error as ActixError, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Generate a JWT token for a user
pub fn generate_token(username: &str) -> String {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "bssoss-secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(8))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Token creation failed")
}

/// Validate a JWT token from the request
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
