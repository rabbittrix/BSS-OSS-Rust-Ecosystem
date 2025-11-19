//! Unit tests for Audit Logging

#[cfg(test)]
mod tests {
    use security::audit::AuditLogger;
    use security::models::{AuditEventType, AuditResult};
    use test_utils::database::create_test_pool;
    use uuid::Uuid;

    async fn setup() -> AuditLogger {
        use test_utils::database::run_test_migrations;
        let pool = create_test_pool()
            .await
            .expect("Failed to create test pool");
        run_test_migrations(&pool)
            .await
            .expect("Failed to run test migrations");
        AuditLogger::new(pool)
    }

    #[tokio::test]
    async fn test_log_authentication() {
        let logger = setup().await;
        let identity_id = Uuid::new_v4();

        let log_id = logger
            .log_authentication(
                Some(identity_id),
                Some("test-user".to_string()),
                AuditResult::Success,
                Some("127.0.0.1".to_string()),
                Some("test-agent".to_string()),
                None,
            )
            .await
            .expect("Failed to log authentication");

        assert!(!log_id.is_nil());
    }

    #[tokio::test]
    async fn test_log_authorization() {
        let logger = setup().await;
        let identity_id = Uuid::new_v4();

        let log_id = logger
            .log_authorization(
                Some(identity_id),
                Some("test-user".to_string()),
                "catalog".to_string(),
                "read".to_string(),
                AuditResult::Success,
                Some("127.0.0.1".to_string()),
                Some("test-agent".to_string()),
                None,
            )
            .await
            .expect("Failed to log authorization");

        assert!(!log_id.is_nil());
    }

    #[tokio::test]
    async fn test_log_oauth_token_issued() {
        let logger = setup().await;
        let identity_id = Uuid::new_v4();

        let log_id = logger
            .log_oauth_token_issued(
                Some(identity_id),
                "test-client".to_string(),
                vec!["openid".to_string(), "profile".to_string()],
                Some("127.0.0.1".to_string()),
                Some("test-agent".to_string()),
            )
            .await
            .expect("Failed to log OAuth token issued");

        assert!(!log_id.is_nil());
    }

    #[tokio::test]
    async fn test_get_identity_logs() {
        let logger = setup().await;
        let identity_id = Uuid::new_v4();

        logger
            .log_authentication(
                Some(identity_id),
                None,
                AuditResult::Success,
                None,
                None,
                None,
            )
            .await
            .expect("Failed to log event");

        let logs = logger
            .get_identity_logs(identity_id, Some(10))
            .await
            .expect("Failed to get identity logs");

        assert!(!logs.is_empty());
        assert_eq!(logs[0].event_type, AuditEventType::Authentication);
    }
}
