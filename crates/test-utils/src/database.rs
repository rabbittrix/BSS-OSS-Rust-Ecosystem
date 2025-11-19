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
        Err(sqlx::Error::Database(db_err))
            if db_err.code() == Some(std::borrow::Cow::Borrowed("3D000")) =>
        {
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
pub async fn run_test_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    use std::fs;
    use std::path::PathBuf;

    // Find migrations directory - try multiple possible locations
    let migrations_dir = find_migrations_dir()?;

    // Get all migration files and sort them
    let mut migration_files: Vec<PathBuf> = fs::read_dir(&migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "sql" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    migration_files.sort();

    // Execute each migration file
    for migration_file in migration_files {
        let sql = fs::read_to_string(&migration_file).map_err(|e| {
            sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read migration file {:?}: {}", migration_file, e),
            ))
        })?;

        // Split SQL by semicolons and execute each statement
        // PostgreSQL allows multiple statements in a single query
        sqlx::query(&sql).execute(pool).await?;
    }

    Ok(())
}

/// Find the migrations directory by searching from current directory up to project root
fn find_migrations_dir() -> Result<std::path::PathBuf, sqlx::Error> {
    let mut current_dir = std::env::current_dir().map_err(|e| {
        sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get current directory: {}", e),
        ))
    })?;

    // Search up to 5 levels for migrations directory
    for _ in 0..5 {
        let migrations_path = current_dir.join("migrations");
        if migrations_path.exists() && migrations_path.is_dir() {
            return Ok(migrations_path);
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    Err(sqlx::Error::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find migrations directory",
    )))
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
