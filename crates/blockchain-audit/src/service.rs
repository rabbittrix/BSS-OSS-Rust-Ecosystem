//! Blockchain Audit Service

use crate::chain::BlockchainAuditChain;
use crate::error::BlockchainAuditError;
use audit_logging::models::{AuditEventType, AuditLogEntry, AuditResult};
use chrono::Utc;
use uuid::Uuid;

/// Blockchain Audit Service
pub struct BlockchainAuditService {
    chain: BlockchainAuditChain,
}

impl BlockchainAuditService {
    /// Create a new blockchain audit service
    pub async fn new(difficulty: usize) -> Self {
        let chain = BlockchainAuditChain::new(difficulty);
        chain.initialize().await;
        Self { chain }
    }

    /// Record an audit event to the blockchain
    #[allow(clippy::too_many_arguments)]
    pub async fn record_audit_event(
        &self,
        event_type: AuditEventType,
        identity_id: Option<Uuid>,
        user_id: Option<String>,
        resource: Option<String>,
        action: Option<String>,
        result: AuditResult,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<serde_json::Value>,
    ) -> Result<Uuid, BlockchainAuditError> {
        let entry = AuditLogEntry {
            id: Uuid::new_v4(),
            event_type,
            identity_id,
            user_id,
            resource,
            action,
            result,
            ip_address,
            user_agent,
            details,
            timestamp: Utc::now(),
        };

        self.chain.add_audit_entry(entry.clone()).await?;

        // Create block if we have enough entries (or immediately for critical events)
        let should_create_block = matches!(
            event_type,
            AuditEventType::SecurityPolicyViolation
                | AuditEventType::Authentication
                | AuditEventType::Authorization
        );

        if should_create_block {
            self.chain.create_block().await?;
        }

        Ok(entry.id)
    }

    /// Force create a block with pending entries
    pub async fn finalize_block(&self) -> Result<(), BlockchainAuditError> {
        self.chain.create_block().await
    }

    /// Validate the blockchain
    pub async fn validate(&self) -> Result<(), BlockchainAuditError> {
        self.chain.validate_chain().await
    }

    /// Get audit trail for an entity
    pub async fn get_audit_trail(
        &self,
        entity_id: Uuid,
    ) -> Result<Vec<AuditLogEntry>, BlockchainAuditError> {
        Ok(self.chain.get_entries_for_entity(entity_id).await)
    }

    /// Get all blocks
    pub async fn get_all_blocks(&self) -> Vec<crate::block::AuditBlock> {
        self.chain.get_all_blocks().await
    }

    /// Get block by index
    pub async fn get_block(
        &self,
        index: u64,
    ) -> Result<Option<crate::block::AuditBlock>, BlockchainAuditError> {
        Ok(self.chain.get_block(index).await)
    }

    /// Get chain statistics
    pub async fn get_statistics(&self) -> ChainStatistics {
        let blocks = self.chain.get_all_blocks().await;
        let total_entries: usize = blocks.iter().map(|b| b.audit_entries.len()).sum();

        ChainStatistics {
            total_blocks: blocks.len(),
            total_entries,
            chain_length: self.chain.length().await,
        }
    }
}

/// Chain statistics
#[derive(Debug, Clone)]
pub struct ChainStatistics {
    pub total_blocks: usize,
    pub total_entries: usize,
    pub chain_length: usize,
}
