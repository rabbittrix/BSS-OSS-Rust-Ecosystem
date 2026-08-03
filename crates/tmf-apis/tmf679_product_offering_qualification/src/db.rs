//! Database operations for TMF679 Product Offering Qualification

use crate::models::{
    CreateProductOfferingQualificationRequest, ProductOfferingQualification,
    ProductOfferingQualificationItem, ProductOfferingRef, QualificationResult,
};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_result(s: &str) -> QualificationResult {
    match s.to_uppercase().as_str() {
        "QUALIFIED" => QualificationResult::Qualified,
        "ALTERNATE" => QualificationResult::Alternate,
        _ => QualificationResult::Unqualified,
    }
}

fn result_to_string(r: &QualificationResult) -> &'static str {
    match r {
        QualificationResult::Qualified => "QUALIFIED",
        QualificationResult::Unqualified => "UNQUALIFIED",
        QualificationResult::Alternate => "ALTERNATE",
    }
}

/// Simple eligibility: premium segment always qualifies; others qualify unless offering name contains "enterprise".
fn evaluate(
    req: &CreateProductOfferingQualificationRequest,
) -> (QualificationResult, Option<String>) {
    let segment = req
        .customer_segment
        .as_deref()
        .unwrap_or("")
        .to_ascii_lowercase();
    if segment == "premium" || segment == "enterprise" {
        return (QualificationResult::Qualified, None);
    }
    if req
        .product_offering_name
        .to_ascii_lowercase()
        .contains("enterprise")
    {
        return (
            QualificationResult::Unqualified,
            Some("Offering requires enterprise segment".into()),
        );
    }
    (QualificationResult::Qualified, None)
}

pub async fn list_qualifications(
    pool: &Pool<Postgres>,
) -> TmfResult<Vec<ProductOfferingQualification>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, href, last_update, state, provide_alternative,
                provide_unavailability_reason, qualification_result, product_offering_id,
                product_offering_name, eligibility_reason, customer_id, requested_date
         FROM product_offering_qualifications ORDER BY last_update DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(rows.into_iter().map(row_to_qualification).collect())
}

pub async fn get_qualification_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<ProductOfferingQualification>> {
    let row = sqlx::query(
        "SELECT id, name, description, version, href, last_update, state, provide_alternative,
                provide_unavailability_reason, qualification_result, product_offering_id,
                product_offering_name, eligibility_reason, customer_id, requested_date
         FROM product_offering_qualifications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;
    Ok(row.map(row_to_qualification))
}

fn row_to_qualification(row: sqlx::postgres::PgRow) -> ProductOfferingQualification {
    let result = parse_result(&row.get::<String, _>("qualification_result"));
    let offering_id: Uuid = row.get("product_offering_id");
    let offering_name: String = row.get("product_offering_name");
    let reason: Option<String> = row.get("eligibility_reason");
    ProductOfferingQualification {
        base: tmf_apis_core::BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
            valid_for: None,
            last_update: row.get("last_update"),
        },
        state: row.get("state"),
        provide_alternative: row.get("provide_alternative"),
        provide_unavailability_reason: row.get("provide_unavailability_reason"),
        qualification_result: Some(result.clone()),
        product_offering_qualification_item: Some(vec![ProductOfferingQualificationItem {
            product_offering: ProductOfferingRef {
                id: offering_id,
                href: Some(format!(
                    "/tmf-api/productCatalogManagement/v4/productOffering/{}",
                    offering_id
                )),
                name: offering_name,
            },
            qualification_result: result,
            eligibility_unavailability_reason: reason,
        }]),
        customer_id: row.get("customer_id"),
        requested_date: row.get("requested_date"),
    }
}

pub async fn create_qualification(
    pool: &Pool<Postgres>,
    req: CreateProductOfferingQualificationRequest,
) -> TmfResult<ProductOfferingQualification> {
    let (result, reason) = evaluate(&req);
    let id = Uuid::new_v4();
    let href = format!(
        "/tmf-api/productOfferingQualification/v4/productOfferingQualification/{}",
        id
    );
    sqlx::query(
        "INSERT INTO product_offering_qualifications
         (id, name, description, version, href, last_update, state, provide_alternative,
          provide_unavailability_reason, qualification_result, product_offering_id,
          product_offering_name, eligibility_reason, customer_id, requested_date)
         VALUES ($1,$2,$3,'1.0',$4,NOW(),'DONE',$5,$6,$7,$8,$9,$10,$11,NOW())",
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&href)
    .bind(req.provide_alternative)
    .bind(req.provide_unavailability_reason)
    .bind(result_to_string(&result))
    .bind(req.product_offering_id)
    .bind(&req.product_offering_name)
    .bind(&reason)
    .bind(req.customer_id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;
    get_qualification_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("qualification".into()))
}
