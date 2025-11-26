//! Blockchain Block Structure

use crate::error::BlockchainAuditError;
use audit_logging::models::AuditLogEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Audit Block in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBlock {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub hash: String,
    pub audit_entries: Vec<AuditLogEntry>,
    pub nonce: u64,
}

impl AuditBlock {
    /// Create a new block
    pub fn new(index: u64, previous_hash: String, audit_entries: Vec<AuditLogEntry>) -> Self {
        let timestamp = Utc::now();
        let mut block = Self {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            audit_entries,
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    /// Calculate the hash of the block
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            self.previous_hash,
            serde_json::to_string(&self.audit_entries).unwrap_or_default(),
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Mine the block (proof of work)
    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);

        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    /// Validate the block
    pub fn validate(&self, previous_hash: &str) -> Result<(), BlockchainAuditError> {
        // Validate hash matches content
        let calculated_hash = self.calculate_hash();
        if calculated_hash != self.hash {
            return Err(BlockchainAuditError::BlockValidationFailed(
                "Block hash does not match content".to_string(),
            ));
        }

        // Validate previous hash link
        if self.previous_hash != previous_hash {
            return Err(BlockchainAuditError::BlockValidationFailed(
                "Previous hash mismatch".to_string(),
            ));
        }

        // Validate index
        if self.index == 0 && !self.previous_hash.is_empty() {
            return Err(BlockchainAuditError::BlockValidationFailed(
                "Genesis block should have empty previous hash".to_string(),
            ));
        }

        Ok(())
    }
}

/// Genesis block (first block in the chain)
impl Default for AuditBlock {
    fn default() -> Self {
        Self::new(0, String::new(), vec![])
    }
}
