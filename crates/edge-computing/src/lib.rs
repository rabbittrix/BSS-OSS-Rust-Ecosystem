//! Edge Computing Support
//!
//! Provides edge computing capabilities for distributed processing:
//! - Edge node management
//! - Task distribution and load balancing
//! - Edge-to-cloud synchronization
//! - Local processing and caching

pub mod error;
pub mod models;
pub mod node;
pub mod orchestrator;
pub mod sync;

pub use error::EdgeComputingError;
pub use models::*;
pub use node::EdgeNodeManager;
pub use orchestrator::EdgeOrchestrator;
pub use sync::EdgeSyncService;
