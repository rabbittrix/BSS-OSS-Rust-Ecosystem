//! Blockchain Integration for Audit Trails
//!
//! Provides immutable audit trail capabilities using blockchain technology.
//! Ensures tamper-proof audit logs for compliance and security.

pub mod block;
pub mod chain;
pub mod error;
pub mod service;

pub use block::AuditBlock;
pub use chain::BlockchainAuditChain;
pub use error::BlockchainAuditError;
pub use service::BlockchainAuditService;
