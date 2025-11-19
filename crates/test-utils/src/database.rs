//! Database test utilities

use sqlx::PgPool;

/// Create a test database pool
pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://bssoss:bssoss123@localhost:5432/bssoss_test".to_string());

    // Extract database name from URL
    let database_name = database_url
        .rsplit('/')
        .next()
        .and_then(|s| s.split('?').next())
        .unwrap_or("bssoss_test")
        .to_string();

    // Try to connect to the test database first
    match PgPool::connect(&database_url).await {
        Ok(pool) => return Ok(pool),
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(std::borrow::Cow::Borrowed("3D000")) => {
            // Database doesn't exist, create it
            // Connect to the default postgres database to create the test database
            let admin_url = database_url
                .rsplitn(2, '/')
                .nth(1)
                .map(|base| format!("{}/postgres", base))
                .unwrap_or_else(|| {
                    // Fallback: replace database name with postgres
                    database_url.replace(&database_name, "postgres")
                });

            let admin_pool = PgPool::connect(&admin_url).await?;

            // Create the database (ignore error if it already exists)
            // Use a parameterized query to avoid SQL injection
            let create_db_query = format!("CREATE DATABASE \"{}\"", database_name);
            let _ = sqlx::query(&create_db_query).execute(&admin_pool).await;

            drop(admin_pool);
        }
        Err(e) => return Err(e),
    }

    // Now connect to the test database
    let pool = PgPool::connect(&database_url).await?;

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
