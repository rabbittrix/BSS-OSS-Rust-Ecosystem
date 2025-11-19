//! Main Order Orchestrator

use crate::decomposition::OrderDecomposer;
use crate::state::{FulfillmentContext, FulfillmentState};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tmf622_ordering::models::ProductOrder;
use uuid::Uuid;

/// Order orchestrator interface
#[async_trait]
pub trait OrderOrchestratorTrait: Send + Sync {
    /// Start orchestrating a product order
    async fn orchestrate(&self, product_order: ProductOrder) -> Result<Uuid, OrchestratorError>;

    /// Get fulfillment context
    async fn get_context(&self, order_id: Uuid) -> Result<FulfillmentContext, OrchestratorError>;

    /// Update task state
    async fn update_task_state(
        &self,
        task_id: Uuid,
        state: FulfillmentState,
    ) -> Result<(), OrchestratorError>;

    /// Process ready tasks
    async fn process_ready_tasks(&self, order_id: Uuid) -> Result<(), OrchestratorError>;
}

/// Order orchestrator implementation
pub struct OrderOrchestrator {
    #[allow(dead_code)]
    pool: Arc<PgPool>,
    // In production, you'd have event bus, external service clients, etc.
}

impl OrderOrchestrator {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl OrderOrchestratorTrait for OrderOrchestrator {
    async fn orchestrate(&self, product_order: ProductOrder) -> Result<Uuid, OrchestratorError> {
        // Decompose the order
        let decomposition = OrderDecomposer::decompose(&product_order);

        // Create fulfillment context
        let mut context = FulfillmentContext::new(product_order.base.id);
        for task in decomposition.tasks {
            context.add_task(task);
        }
        context.state = FulfillmentState::Decomposing;

        // Store context (in production, persist to database)
        // For now, we'll just return the order ID
        // In a real implementation, you'd:
        // 1. Store context in database
        // 2. Publish events to event bus
        // 3. Start processing tasks

        Ok(product_order.base.id)
    }

    async fn get_context(&self, _order_id: Uuid) -> Result<FulfillmentContext, OrchestratorError> {
        // In production, load from database
        Err(OrchestratorError::NotImplemented)
    }

    async fn update_task_state(
        &self,
        _task_id: Uuid,
        _state: FulfillmentState,
    ) -> Result<(), OrchestratorError> {
        // In production, update database and publish events
        Err(OrchestratorError::NotImplemented)
    }

    async fn process_ready_tasks(&self, _order_id: Uuid) -> Result<(), OrchestratorError> {
        // In production:
        // 1. Load context
        // 2. Get ready tasks
        // 3. For each task, create corresponding order (service/resource)
        // 4. Update task state
        // 5. Publish events
        Err(OrchestratorError::NotImplemented)
    }
}

/// Orchestrator errors
#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Order not found")]
    OrderNotFound,
    #[error("Task not found")]
    TaskNotFound,
    #[error("Invalid state transition")]
    InvalidStateTransition,
    #[error("Not implemented")]
    NotImplemented,
    #[error("External service error: {0}")]
    ExternalService(String),
}
