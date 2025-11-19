//! Network Selection Policies

use serde::{Deserialize, Serialize};

/// Network type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NetworkType {
    Fiber,
    FiveG,
    Fwa, // Fixed Wireless Access
    Lte,
    Dsl,
}

/// Network selection policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSelectionPolicy {
    pub preferred_networks: Vec<NetworkType>,
    pub fallback_networks: Vec<NetworkType>,
    pub selection_rules: serde_json::Value,
}
