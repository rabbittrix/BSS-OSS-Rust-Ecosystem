//! Test helper functions

use actix_web::test;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a test JWT token
pub fn create_test_token(user_id: Uuid) -> String {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    // Use a test secret - in production, use environment variable
    let secret = b"test-secret-key-for-testing-only";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .expect("Failed to create test token")
}

/// Create test HTTP request with authentication
pub fn create_test_request(method: &str, path: &str, body: Option<&str>) -> test::TestRequest {
    use actix_web::http::{header::HeaderValue, Method};

    // Add test token
    let test_user_id = Uuid::new_v4();
    let token = create_test_token(test_user_id);
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", token))
        .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid"));

    let mut req = test::TestRequest::default()
        .method(Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET))
        .uri(path)
        .insert_header(("Content-Type", HeaderValue::from_static("application/json")))
        .insert_header(("Authorization", auth_header_value));

    if let Some(body_str) = body {
        req = req.set_payload(body_str.to_string());
    }

    req
}

/// Setup test database pool (for integration tests)
pub async fn setup_test_db() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5432/bss_oss_test".to_string()
    });

    PgPool::connect(&database_url).await
}

/// Cleanup test data
pub async fn cleanup_test_data(pool: &PgPool, table: &str) -> Result<(), sqlx::Error> {
    sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
        .execute(pool)
        .await?;
    Ok(())
}

/// Assert JSON response structure
pub fn assert_json_response(
    response: &str,
    expected_fields: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_str(response)?;

    if let Some(obj) = json.as_object() {
        for field in expected_fields {
            if !obj.contains_key(*field) {
                return Err(format!("Missing expected field: {}", field).into());
            }
        }
    } else {
        return Err("Response is not a JSON object".into());
    }

    Ok(())
}
