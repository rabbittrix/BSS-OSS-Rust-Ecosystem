//! Product Catalog Engine (PCM) Framework
//!
//! The Product Catalog Engine is the heart of the BSS, providing business agility
//! through efficient management of pricing, eligibility, and bundling rules.
//!
//! This framework abstracts the complexity of:
//! - Pricing rules and calculations
//! - Product eligibility validation
//! - Bundling and product relationships
//! - Catalog versioning and lifecycle management
//!
//! Built with Rust's safety guarantees to prevent costly billing errors.

pub mod bundling;
pub mod eligibility;
pub mod engine;
pub mod pricing;
pub mod rules;

pub use bundling::*;
pub use eligibility::*;
pub use engine::CatalogEngine;
pub use pricing::*;
pub use rules::*;
