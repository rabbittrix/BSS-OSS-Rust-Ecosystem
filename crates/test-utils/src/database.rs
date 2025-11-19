//! Database test utilities

use sqlx::PgPool;
/// Create a test database pool
pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://bssoss:bssoss123@localhost:5432/bssoss_test".to_string());

    let pool =
        PgPool::connect_with(database_url.parse().expect("Invalid TEST_DATABASE_URL")).await?;

    // Set connection pool options
    Ok(pool)
}

/// Run database migrations for tests
pub async fn run_test_migrations(_pool: &PgPool) -> Result<(), sqlx::Error> {
    // In a real implementation, you would run migrations here
    // For now, we assume migrations are run separately
    Ok(())
}

/// Clean up test database
pub async fn cleanup_test_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Truncate all test tables
    let tables = vec![
        "audit_logs",
        "user_roles",
        "roles",
        "mfa_challenges",
        "mfa_configs",
        "access_tokens",
        "authorization_codes",
        "oauth_clients",
        "partner_settlements",
        "settlement_rules",
        "billing_cycles",
        "tiered_rates",
        "rating_rules",
        "charging_results",
        "network_slices",
        "alarms",
        "identity_credentials",
        "identities",
        "party_roles",
        "parties",
        "usages",
        "resource_orders",
        "resource_inventories",
        "resource_activations",
        "service_activations",
        "service_inventories",
        "service_orders",
        "appointments",
        "customer_usages",
        "customer_bills",
        "customers",
        "product_inventories",
        "product_orders",
        "product_offerings",
        "catalogs",
    ];

    for table in tables {
        let _ = sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
            .execute(pool)
            .await;
    }

    Ok(())
}

/// Create a test transaction
pub async fn with_test_transaction<F, Fut, T>(pool: &PgPool, f: F) -> Result<T, sqlx::Error>
where
    F: FnOnce(&mut sqlx::Transaction<'_, sqlx::Postgres>) -> Fut,
    Fut: std::future::Future<Output = Result<T, sqlx::Error>>,
{
    let mut tx = pool.begin().await?;
    let result = f(&mut tx).await;
    tx.rollback().await?;
    result
}
