//! Unit tests for MFA

#[cfg(test)]
mod tests {
    use security::mfa::MfaService;
    use security::models::MfaMethod;
    use test_utils::database::create_test_pool;
    use uuid::Uuid;

    async fn setup() -> MfaService {
        use test_utils::database::run_test_migrations;
        let pool = create_test_pool()
            .await
            .expect("Failed to create test pool");
        run_test_migrations(&pool)
            .await
            .expect("Failed to run test migrations");
        MfaService::new(pool, "TestIssuer".to_string())
    }

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_enable_totp() {
        let mfa = setup().await;
        let identity_id = Uuid::new_v4();

        let (secret, qr_data) = mfa
            .enable_totp(identity_id)
            .await
            .expect("Failed to enable TOTP");

        assert!(!secret.is_empty());
        assert!(qr_data.contains("otpauth://totp"));
    }

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_enable_sms() {
        let mfa = setup().await;
        let identity_id = Uuid::new_v4();

        mfa.enable_sms(identity_id, "+1234567890".to_string())
            .await
            .expect("Failed to enable SMS MFA");

        let status = mfa
            .get_mfa_status(identity_id)
            .await
            .expect("Failed to get MFA status");

        assert!(status.iter().any(|s| matches!(s.method, MfaMethod::Sms)));
    }

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_enable_email() {
        let mfa = setup().await;
        let identity_id = Uuid::new_v4();

        mfa.enable_email(identity_id, "test@example.com".to_string())
            .await
            .expect("Failed to enable Email MFA");

        let status = mfa
            .get_mfa_status(identity_id)
            .await
            .expect("Failed to get MFA status");

        assert!(status.iter().any(|s| matches!(s.method, MfaMethod::Email)));
    }

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_get_mfa_status() {
        let mfa = setup().await;
        let identity_id = Uuid::new_v4();

        mfa.enable_sms(identity_id, "+1234567890".to_string())
            .await
            .expect("Failed to enable SMS MFA");

        let status = mfa
            .get_mfa_status(identity_id)
            .await
            .expect("Failed to get MFA status");

        assert!(!status.is_empty());
        assert!(status[0].is_enabled);
    }
}
