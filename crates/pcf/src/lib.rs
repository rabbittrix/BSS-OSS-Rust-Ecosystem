//! Policy Control Function (PCF/PCRF) for BSS/OSS Rust Ecosystem
//!
//! The PCF (Policy Control Function), formerly known as PCRF (Policy and Charging Rules Function),
//! is the "brain" of policies in mobile telecommunications networks (3G, 4G, 5G, and 6G).
//!
//! ## Core Functions
//!
//! 1. **Policy Control**: Defines Quality of Service (QoS), bandwidth limits, prioritization, and gating
//! 2. **Charging Rules**: Manages online and offline charging for prepaid and postpaid plans
//! 3. **Quota Management**: Tracks data allowances and triggers speed throttling or notifications
//!
//! ## Key Features
//!
//! - Real-time policy decisions for network traffic
//! - Diameter protocol support (Gx, Gy, Gz interfaces)
//! - Support for 3G, 4G, 5G, and 6G networks
//! - AI/ML integration hooks for intelligent policy decisions
//! - Zero-rating support (unlimited apps/services)
//! - Network congestion management
//! - Real-time quota monitoring and notifications
//!
//! ## Example Usage
//!
//! ```rust
//! use bss_oss_pcf::{PcfEngine, PolicyRequest, NetworkGeneration};
//! use bss_oss_pcf::pcf_engine::PcfEngineTrait;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let pcf = PcfEngine::new();
//!
//! let request = PolicyRequest {
//!     subscriber_id: "1234567890".to_string(),
//!     imsi: "123456789012345".to_string(),
//!     cpf: Some("123.456.789-09".to_string()),
//!     network_generation: NetworkGeneration::FourG,
//!     apn: "internet".to_string(),
//!     service_type: "video_streaming".to_string(),
//!     application_id: Some("youtube.com".to_string()),
//!     location: None,
//!     time_of_day: None,
//! };
//!
//! let policy = pcf.evaluate_policy(&request).await?;
//! println!("QoS Priority: {}", policy.qos.priority);
//! println!("Access Granted: {}", policy.access_granted);
//! # Ok(())
//! # }
//! ```

pub mod ai;
pub mod charging;
pub mod cpf;
pub mod diameter;
pub mod error;
pub mod models;
pub mod pcf_engine;
pub mod policy;
pub mod quota;
pub mod tax_id;

pub use cpf::Cpf;
pub use error::PcfError;
pub use models::*;
pub use pcf_engine::PcfEngine;
pub use tax_id::{TaxId, TaxIdCountry};
