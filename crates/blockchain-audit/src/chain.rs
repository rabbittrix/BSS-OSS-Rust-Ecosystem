//! Blockchain Chain Management

use crate::block::AuditBlock;
use crate::error::BlockchainAuditError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Blockchain Audit Chain
pub struct BlockchainAuditChain {
    chain: Arc<RwLock<Vec<AuditBlock>>>,
    difficulty: usize,
    pending_entries: Arc<RwLock<Vec<audit_logging::models::AuditLogEntry>>>,
}

impl BlockchainAuditChain {
    /// Create a new blockchain
    pub fn new(difficulty: usize) -> Self {
        // Create genesis block (will be added on first access)
        Self {
            chain: Arc::new(RwLock::new(Vec::new())),
            difficulty,
            pending_entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize with genesis block
    pub async fn initialize(&self) {
        let mut chain = self.chain.write().await;
        if chain.is_empty() {
            let genesis = AuditBlock::default();
            chain.push(genesis);
        }
    }

    /// Get the latest block
    pub async fn get_latest_block(&self) -> Option<AuditBlock> {
        self.chain.read().await.last().cloned()
    }

    /// Add a new audit entry to pending entries
    pub async fn add_audit_entry(
        &self,
        entry: audit_logging::models::AuditLogEntry,
    ) -> Result<(), BlockchainAuditError> {
        self.pending_entries.write().await.push(entry);
        Ok(())
    }

    /// Create a new block with pending entries
    pub async fn create_block(&self) -> Result<(), BlockchainAuditError> {
        let entries: Vec<audit_logging::models::AuditLogEntry> = {
            let mut pending = self.pending_entries.write().await;
            if pending.is_empty() {
                return Ok(()); // No entries to add
            }
            pending.drain(..).collect()
        };

        let chain = self.chain.read().await;
        let previous_block = chain.last().ok_or_else(|| {
            BlockchainAuditError::ChainValidationFailed("No previous block found".to_string())
        })?;

        let previous_hash = previous_block.hash.clone();
        let index = chain.len() as u64;
        drop(chain);

        let mut new_block = AuditBlock::new(index, previous_hash, entries);
        new_block.mine_block(self.difficulty);

        let mut chain = self.chain.write().await;
        chain.push(new_block);

        Ok(())
    }

    /// Validate the entire chain
    pub async fn validate_chain(&self) -> Result<(), BlockchainAuditError> {
        let chain = self.chain.read().await;

        if chain.is_empty() {
            return Err(BlockchainAuditError::ChainValidationFailed(
                "Chain is empty".to_string(),
            ));
        }

        // Validate genesis block
        let genesis = &chain[0];
        if genesis.index != 0 || !genesis.previous_hash.is_empty() {
            return Err(BlockchainAuditError::ChainValidationFailed(
                "Invalid genesis block".to_string(),
            ));
        }

        // Validate each block
        for i in 1..chain.len() {
            let block = &chain[i];
            let previous_block = &chain[i - 1];

            block.validate(&previous_block.hash)?;
        }

        Ok(())
    }

    /// Get all blocks
    pub async fn get_all_blocks(&self) -> Vec<AuditBlock> {
        self.chain.read().await.clone()
    }

    /// Get block by index
    pub async fn get_block(&self, index: u64) -> Option<AuditBlock> {
        self.chain.read().await.get(index as usize).cloned()
    }

    /// Get audit entries for a specific entity
    pub async fn get_entries_for_entity(
        &self,
        entity_id: uuid::Uuid,
    ) -> Vec<audit_logging::models::AuditLogEntry> {
        let chain = self.chain.read().await;
        let mut entries = Vec::new();

        for block in chain.iter() {
            for entry in &block.audit_entries {
                if entry.identity_id == Some(entity_id) {
                    entries.push(entry.clone());
                }
            }
        }

        entries
    }

    /// Get chain length
    pub async fn length(&self) -> usize {
        self.chain.read().await.len()
    }
}
