//! GraphQL API Layer
//!
//! Provides GraphQL interface for BSS/OSS operations

pub mod resolvers;
pub mod schema;

pub use schema::create_schema;
