//! Unit tests for OAuth 2.0 / OIDC integration

#[cfg(test)]
mod tests {
    use security::models::GrantType;
    use security::oauth::OAuthProvider;
    use sqlx::PgPool;
    use test_utils::database::create_test_pool;
    use uuid::Uuid;

    async fn setup() -> (PgPool, OAuthProvider) {
        let pool = create_test_pool()
            .await
            .expect("Failed to create test pool");
        let provider = OAuthProvider::new(pool.clone(), "http://localhost:8080".to_string());
        (pool, provider)
    }

    #[tokio::test]
    async fn test_register_oauth_client() {
        let (_pool, provider) = setup().await;
        let identity_id = Uuid::new_v4();

        let client = provider
            .register_client(
                "test-client".to_string(),
                "test-secret".to_string(),
                vec!["http://localhost:3000/callback".to_string()],
                vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
                vec!["openid".to_string(), "profile".to_string()],
                identity_id,
            )
            .await
            .expect("Failed to register client");

        assert_eq!(client.client_id, "test-client");
        assert!(!client.client_secret_hash.is_empty());
        assert_eq!(client.redirect_uris.len(), 1);
        assert_eq!(client.grant_types.len(), 2);
    }

    #[tokio::test]
    async fn test_validate_client_credentials() {
        let (_pool, provider) = setup().await;
        let identity_id = Uuid::new_v4();

        let _client = provider
            .register_client(
                "test-client-2".to_string(),
                "test-secret-2".to_string(),
                vec!["http://localhost:3000/callback".to_string()],
                vec![GrantType::ClientCredentials],
                vec!["api".to_string()],
                identity_id,
            )
            .await
            .expect("Failed to register client");

        let validated = provider
            .validate_client("test-client-2", "test-secret-2")
            .await
            .expect("Failed to validate client");

        assert_eq!(validated.client_id, "test-client-2");
    }

    #[tokio::test]
    async fn test_generate_authorization_code() {
        let (_pool, provider) = setup().await;
        let user_id = Uuid::new_v4();

        let code = provider
            .generate_authorization_code(
                "test-client".to_string(),
                user_id,
                "http://localhost:3000/callback".to_string(),
                vec!["openid".to_string()],
                None,
                None,
            )
            .await
            .expect("Failed to generate authorization code");

        assert!(!code.code.is_empty());
        assert_eq!(code.client_id, "test-client");
        assert_eq!(code.user_id, user_id);
    }

    #[tokio::test]
    async fn test_generate_access_token() {
        let (_pool, provider) = setup().await;

        let token = provider
            .generate_client_credentials_token("test-client", &["api".to_string()])
            .await
            .expect("Failed to generate access token");

        assert!(!token.token.is_empty());
        assert_eq!(token.token_type, "Bearer");
        assert!(token.expires_in > 0);
    }

    #[tokio::test]
    async fn test_validate_access_token() {
        let (_pool, provider) = setup().await;

        let token = provider
            .generate_client_credentials_token("test-client", &["api".to_string()])
            .await
            .expect("Failed to generate access token");

        let validated = provider
            .validate_access_token(&token.token)
            .await
            .expect("Failed to validate access token");

        assert_eq!(validated.token, token.token);
        assert_eq!(validated.client_id, "test-client");
    }

    #[tokio::test]
    async fn test_oidc_discovery_document() {
        let (_pool, provider) = setup().await;

        let discovery = provider.get_discovery_document();

        assert_eq!(discovery["issuer"], "http://localhost:8080");
        assert!(discovery["authorization_endpoint"].is_string());
        assert!(discovery["token_endpoint"].is_string());
    }
}
