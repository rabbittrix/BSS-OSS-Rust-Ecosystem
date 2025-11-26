//! GraphQL resolvers for TMF APIs

use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

/// Query root for GraphQL
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check
    async fn health(&self) -> &str {
        "ok"
    }

    /// Get catalog by ID
    async fn catalog(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Catalog>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&id.to_string())
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let row = sqlx::query_as::<_, CatalogRow>(
            "SELECT id, name, description, version, lifecycle_status, href, last_update, created_at
             FROM catalogs WHERE id = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// List all catalogs
    async fn catalogs(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<Catalog>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(100).min(1000);

        let rows = sqlx::query_as::<_, CatalogRow>(
            "SELECT id, name, description, version, lifecycle_status, href, last_update, created_at
             FROM catalogs ORDER BY created_at DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Get product offering by ID
    async fn product_offering(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ProductOffering>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&id.to_string())
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let row = sqlx::query_as::<_, ProductOfferingRow>(
            "SELECT id, name, description, version, lifecycle_status, href, last_update, created_at,
                    is_sellable, is_bundle
             FROM product_offerings WHERE id = $1"
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// List all product offerings
    async fn product_offerings(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<ProductOffering>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(100).min(1000);

        let rows = sqlx::query_as::<_, ProductOfferingRow>(
            "SELECT id, name, description, version, lifecycle_status, href, last_update, created_at,
                    is_sellable, is_bundle
             FROM product_offerings ORDER BY created_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Get customer by ID
    async fn customer(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Customer>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&id.to_string())
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let row = sqlx::query_as::<_, CustomerRow>(
            "SELECT id, name, description, version, state, status, href, last_update, created_at
             FROM customers WHERE id = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// List all customers
    async fn customers(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<Customer>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(100).min(1000);

        let rows = sqlx::query_as::<_, CustomerRow>(
            "SELECT id, name, description, version, state, status, href, last_update, created_at
             FROM customers ORDER BY created_at DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

/// GraphQL Catalog type
#[derive(async_graphql::SimpleObject)]
pub struct Catalog {
    #[graphql(name = "id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: String,
    pub href: Option<String>,
    pub last_update: Option<String>,
    pub created_at: String,
}

/// GraphQL Product Offering type
#[derive(async_graphql::SimpleObject)]
pub struct ProductOffering {
    #[graphql(name = "id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub lifecycle_status: String,
    pub href: Option<String>,
    pub last_update: Option<String>,
    pub created_at: String,
    pub is_sellable: bool,
    pub is_bundle: bool,
}

/// GraphQL Customer type
#[derive(async_graphql::SimpleObject)]
pub struct Customer {
    #[graphql(name = "id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub state: String,
    pub status: Option<String>,
    pub href: Option<String>,
    pub last_update: Option<String>,
    pub created_at: String,
}

/// Internal database row types
#[derive(sqlx::FromRow)]
struct CatalogRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    version: Option<String>,
    lifecycle_status: String,
    href: Option<String>,
    last_update: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct ProductOfferingRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    version: Option<String>,
    lifecycle_status: String,
    href: Option<String>,
    last_update: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    is_sellable: bool,
    is_bundle: bool,
}

#[derive(sqlx::FromRow)]
struct CustomerRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    version: Option<String>,
    state: String,
    status: Option<String>,
    href: Option<String>,
    last_update: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<CatalogRow> for Catalog {
    fn from(row: CatalogRow) -> Self {
        Self {
            id: row.id.to_string(),
            name: row.name,
            description: row.description,
            version: row.version,
            lifecycle_status: row.lifecycle_status,
            href: row.href,
            last_update: row.last_update.map(|dt| dt.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
        }
    }
}

impl From<ProductOfferingRow> for ProductOffering {
    fn from(row: ProductOfferingRow) -> Self {
        Self {
            id: row.id.to_string(),
            name: row.name,
            description: row.description,
            version: row.version,
            lifecycle_status: row.lifecycle_status,
            href: row.href,
            last_update: row.last_update.map(|dt| dt.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
            is_sellable: row.is_sellable,
            is_bundle: row.is_bundle,
        }
    }
}

impl From<CustomerRow> for Customer {
    fn from(row: CustomerRow) -> Self {
        Self {
            id: row.id.to_string(),
            name: row.name,
            description: row.description,
            version: row.version,
            state: row.state,
            status: row.status,
            href: row.href,
            last_update: row.last_update.map(|dt| dt.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
        }
    }
}
