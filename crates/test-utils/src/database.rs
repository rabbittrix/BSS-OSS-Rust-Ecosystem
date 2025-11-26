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
                .rsplit_once('/')
                .map(|(base, _)| format!("{}/postgres", base))
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
            sqlx::Error::Io(std::io::Error::other(format!(
                "Failed to read migration file {:?}: {}",
                migration_file, e
            )))
        })?;

        // Split SQL into individual statements and execute each one
        // sqlx::query() can only execute one statement at a time
        let statements = split_sql_statements(&sql);
        for (idx, statement) in statements.iter().enumerate() {
            let trimmed = statement.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }

            // Execute statement, providing context on failure
            // Some statements may fail if table doesn't exist, which is non-critical for IF NOT EXISTS
            let result = sqlx::query(trimmed).execute(pool).await;
            if let Err(e) = result {
                let error_msg = e.to_string();
                let is_table_not_found = error_msg.contains("does not exist")
                    || (error_msg.contains("relation") && error_msg.contains("does not exist"));
                let upper = trimmed.trim().to_uppercase();

                // Check if this is a CREATE TABLE statement first (critical)
                // CREATE TABLE must succeed - any error (foreign key, syntax, etc.) is critical
                if upper.starts_with("CREATE TABLE") {
                    // CREATE TABLE is critical - always fail with detailed error
                    // This includes foreign key constraint errors, syntax errors, etc.
                    return Err(sqlx::Error::Io(std::io::Error::other(format!(
                        "Failed to execute CREATE TABLE statement {} in migration {:?}: {}\nStatement (first 500 chars): {}\n\nThis is a critical error. The table was not created.",
                        idx + 1,
                        migration_file.file_name().unwrap_or_default(),
                        e,
                        trimmed.chars().take(500).collect::<String>()
                    ))));
                }

                // Non-critical statements that can fail if table doesn't exist:
                // - COMMENT ON TABLE/INDEX (documentation)
                // - CREATE UNIQUE INDEX (unique indexes can be recreated)
                // - CREATE INDEX IF NOT EXISTS (indexes can be recreated)
                // - CREATE INDEX (indexes can be recreated)
                // Check in order from most specific to least specific
                let is_index_statement = upper.starts_with("CREATE UNIQUE INDEX")
                    || upper.starts_with("CREATE INDEX IF NOT EXISTS")
                    || upper.starts_with("CREATE INDEX");
                let is_comment_statement = upper.starts_with("COMMENT ON");

                let is_non_critical =
                    is_table_not_found && (is_comment_statement || is_index_statement);

                if is_non_critical {
                    // Non-critical error - table/index might not exist, skip
                    continue;
                }

                // For other errors, fail with context
                return Err(sqlx::Error::Io(std::io::Error::other(format!(
                    "Failed to execute statement {} in migration {:?}: {}\nStatement: {}",
                    idx + 1,
                    migration_file.file_name().unwrap_or_default(),
                    e,
                    trimmed.chars().take(200).collect::<String>()
                ))));
            }
        }
    }

    // Verify that critical tables exist after migrations
    // This helps catch cases where CREATE TABLE statements fail silently
    let critical_tables = vec!["identities", "audit_logs", "roles", "user_roles"];
    for table in critical_tables {
        let check_query = format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}')",
            table
        );
        let exists: bool = sqlx::query_scalar(&check_query)
            .fetch_one(pool)
            .await
            .unwrap_or(false);

        if !exists {
            return Err(sqlx::Error::Io(std::io::Error::other(format!(
                "Critical table '{}' does not exist after running migrations. This indicates a CREATE TABLE statement failed or was not executed.",
                table
            ))));
        }
    }

    Ok(())
}

/// Split SQL content into individual statements
/// This is a simple splitter that handles basic cases
fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut string_char = '\0';
    let mut in_comment = false;
    let mut comment_type = CommentType::None;

    #[derive(PartialEq)]
    enum CommentType {
        None,
        SingleLine,
        MultiLine,
    }

    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        let next_ch = if i + 1 < chars.len() {
            Some(chars[i + 1])
        } else {
            None
        };

        // Handle string literals
        // PostgreSQL uses single quotes and escapes by doubling ('')
        if !in_comment && (ch == '\'' || ch == '"') {
            if !in_string {
                in_string = true;
                string_char = ch;
            } else if ch == string_char {
                // Check if it's an escaped quote (doubled)
                if ch == '\'' && next_ch == Some('\'') {
                    // Doubled single quote - still in string
                    current.push(ch);
                    current.push(next_ch.unwrap());
                    i += 2;
                    continue;
                } else {
                    // End of string
                    in_string = false;
                }
            }
            current.push(ch);
            i += 1;
            continue;
        }

        if in_string {
            current.push(ch);
            i += 1;
            continue;
        }

        // Handle comments
        if !in_string {
            if ch == '-' && next_ch == Some('-') && comment_type == CommentType::None {
                in_comment = true;
                comment_type = CommentType::SingleLine;
                current.push(ch);
                current.push(next_ch.unwrap());
                i += 2;
                continue;
            } else if ch == '/' && next_ch == Some('*') && comment_type == CommentType::None {
                in_comment = true;
                comment_type = CommentType::MultiLine;
                current.push(ch);
                current.push(next_ch.unwrap());
                i += 2;
                continue;
            } else if in_comment {
                if comment_type == CommentType::SingleLine && ch == '\n' {
                    in_comment = false;
                    comment_type = CommentType::None;
                } else if comment_type == CommentType::MultiLine
                    && ch == '*'
                    && next_ch == Some('/')
                {
                    in_comment = false;
                    comment_type = CommentType::None;
                    current.push(ch);
                    current.push(next_ch.unwrap());
                    i += 2;
                    continue;
                }
                current.push(ch);
                i += 1;
                continue;
            }
        }

        // Handle statement termination
        if ch == ';' && !in_string && !in_comment {
            current.push(ch);
            statements.push(current.clone());
            current.clear();
            i += 1;
            continue;
        }

        current.push(ch);
        i += 1;
    }

    // Add remaining content as a statement if it's not empty
    let trimmed = current.trim();
    if !trimmed.is_empty() && !trimmed.starts_with("--") {
        statements.push(current);
    }

    statements
}

/// Find the migrations directory by searching from current directory up to project root
fn find_migrations_dir() -> Result<std::path::PathBuf, sqlx::Error> {
    let mut current_dir = std::env::current_dir().map_err(|e| {
        sqlx::Error::Io(std::io::Error::other(format!(
            "Failed to get current directory: {}",
            e
        )))
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
