//! Catalog Versioning System
//!
//! Manages catalog versions with content snapshots, publish/rollback, and diffs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bundling::{Bundle, ProductRelationship};
use crate::eligibility::EligibilityRule;
use crate::pricing::PricingRule;
use crate::rules::CatalogRule;

/// Immutable snapshot of catalog content at a version point.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogSnapshot {
    pub pricing_rules: Vec<PricingRule>,
    pub eligibility_rules: Vec<EligibilityRule>,
    pub bundles: Vec<Bundle>,
    pub catalog_rules: Vec<CatalogRule>,
    pub relationships: Vec<ProductRelationship>,
}

/// Catalog version metadata + optional content snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogVersion {
    pub id: Uuid,
    pub catalog_id: Uuid,
    pub version: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub is_active: bool,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    /// Content captured when the version was created or updated.
    #[serde(default)]
    pub snapshot: CatalogSnapshot,
}

/// Version manager for catalogs
#[derive(Default)]
pub struct VersionManager {
    versions: Vec<CatalogVersion>,
}

impl VersionManager {
    /// Create a new version manager
    pub fn new() -> Self {
        Self {
            versions: Vec::new(),
        }
    }

    /// Create a new version with a content snapshot.
    pub fn create_version(
        &mut self,
        catalog_id: Uuid,
        version: String,
        description: Option<String>,
        created_by: Option<Uuid>,
        snapshot: CatalogSnapshot,
    ) -> CatalogVersion {
        let catalog_version = CatalogVersion {
            id: Uuid::new_v4(),
            catalog_id,
            version,
            description,
            created_at: Utc::now(),
            created_by,
            is_active: false,
            is_published: false,
            published_at: None,
            metadata: None,
            snapshot,
        };
        self.versions.push(catalog_version.clone());
        catalog_version
    }

    /// Publish a version (makes it the sole active version for the catalog).
    pub fn publish_version(&mut self, version_id: Uuid) -> Result<&CatalogVersion, String> {
        let catalog_id = {
            let version = self
                .versions
                .iter()
                .find(|v| v.id == version_id)
                .ok_or_else(|| "Version not found".to_string())?;
            version.catalog_id
        };

        for v in self.versions.iter_mut() {
            if v.catalog_id == catalog_id && v.id != version_id {
                v.is_active = false;
            }
        }

        let version = self
            .versions
            .iter_mut()
            .find(|v| v.id == version_id)
            .ok_or_else(|| "Version not found".to_string())?;
        version.is_active = true;
        version.is_published = true;
        version.published_at = Some(Utc::now());
        Ok(version)
    }

    /// Rollback to a previous version (re-publishes it).
    pub fn rollback_to_version(&mut self, version_id: Uuid) -> Result<&CatalogVersion, String> {
        self.publish_version(version_id)
    }

    /// Get active version for a catalog
    pub fn get_active_version(&self, catalog_id: Uuid) -> Option<&CatalogVersion> {
        self.versions
            .iter()
            .find(|v| v.catalog_id == catalog_id && v.is_active)
    }

    /// Get snapshot of the active catalog version.
    pub fn get_active_snapshot(&self, catalog_id: Uuid) -> Option<&CatalogSnapshot> {
        self.get_active_version(catalog_id).map(|v| &v.snapshot)
    }

    /// Get all versions for a catalog
    pub fn get_versions(&self, catalog_id: Uuid) -> Vec<&CatalogVersion> {
        self.versions
            .iter()
            .filter(|v| v.catalog_id == catalog_id)
            .collect()
    }

    /// Compare two versions (content-aware diff).
    pub fn compare_versions(
        &self,
        version_id_1: Uuid,
        version_id_2: Uuid,
    ) -> Result<VersionDiff, String> {
        let v1 = self
            .versions
            .iter()
            .find(|v| v.id == version_id_1)
            .ok_or_else(|| "Version 1 not found".to_string())?;
        let v2 = self
            .versions
            .iter()
            .find(|v| v.id == version_id_2)
            .ok_or_else(|| "Version 2 not found".to_string())?;

        Ok(VersionDiff {
            version_1: v1.clone(),
            version_2: v2.clone(),
            differences: diff_snapshots(&v1.snapshot, &v2.snapshot),
        })
    }
}

fn diff_snapshots(a: &CatalogSnapshot, b: &CatalogSnapshot) -> Vec<String> {
    let mut diffs = Vec::new();
    if a.pricing_rules.len() != b.pricing_rules.len() {
        diffs.push(format!(
            "pricing_rules: {} → {}",
            a.pricing_rules.len(),
            b.pricing_rules.len()
        ));
    }
    if a.eligibility_rules.len() != b.eligibility_rules.len() {
        diffs.push(format!(
            "eligibility_rules: {} → {}",
            a.eligibility_rules.len(),
            b.eligibility_rules.len()
        ));
    }
    if a.bundles.len() != b.bundles.len() {
        diffs.push(format!("bundles: {} → {}", a.bundles.len(), b.bundles.len()));
    }
    if a.catalog_rules.len() != b.catalog_rules.len() {
        diffs.push(format!(
            "catalog_rules: {} → {}",
            a.catalog_rules.len(),
            b.catalog_rules.len()
        ));
    }
    if a.relationships.len() != b.relationships.len() {
        diffs.push(format!(
            "relationships: {} → {}",
            a.relationships.len(),
            b.relationships.len()
        ));
    }

    let a_price_ids: std::collections::HashSet<_> =
        a.pricing_rules.iter().map(|r| r.id).collect();
    let b_price_ids: std::collections::HashSet<_> =
        b.pricing_rules.iter().map(|r| r.id).collect();
    for id in a_price_ids.difference(&b_price_ids) {
        diffs.push(format!("pricing_rule removed: {id}"));
    }
    for id in b_price_ids.difference(&a_price_ids) {
        diffs.push(format!("pricing_rule added: {id}"));
    }

    diffs
}

/// Version comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    pub version_1: CatalogVersion,
    pub version_2: CatalogVersion,
    pub differences: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::{Money, PriceType, PricingRule};

    #[test]
    fn publish_and_diff_snapshots() {
        let catalog_id = Uuid::new_v4();
        let mut vm = VersionManager::new();
        let empty = CatalogSnapshot::default();
        let v1 = vm.create_version(catalog_id, "1.0.0".into(), None, None, empty);

        let mut snap2 = CatalogSnapshot::default();
        snap2.pricing_rules.push(PricingRule {
            id: Uuid::new_v4(),
            product_offering_id: Uuid::new_v4(),
            price_type: PriceType::OneTime,
            base_price: Money {
                value: 10.0,
                unit: "USD".into(),
            },
            priority: 1,
            discount_rules: None,
            valid_for: None,
        });
        let v2 = vm.create_version(
            catalog_id,
            "1.1.0".into(),
            Some("add price".into()),
            None,
            snap2,
        );

        vm.publish_version(v2.id).unwrap();
        assert_eq!(
            vm.get_active_version(catalog_id).unwrap().version,
            "1.1.0"
        );

        let diff = vm.compare_versions(v1.id, v2.id).unwrap();
        assert!(!diff.differences.is_empty());
    }
}
