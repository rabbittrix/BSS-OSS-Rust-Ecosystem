//! Order Decomposition Logic

use crate::state::{FulfillmentTask, TaskType};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tmf622_ordering::models::ProductOrder;
use uuid::Uuid;

/// Order decomposition result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompositionResult {
    pub product_order_id: Uuid,
    pub service_orders: Vec<ServiceOrderSpec>,
    pub resource_orders: Vec<ResourceOrderSpec>,
    pub tasks: Vec<FulfillmentTask>,
}

/// Service order specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrderSpec {
    pub id: Uuid,
    pub product_order_item_id: Uuid,
    pub service_specification_id: Option<Uuid>,
    pub quantity: i32,
    pub action: String, // ADD, MODIFY, DELETE
}

/// Resource order specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceOrderSpec {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub resource_specification_id: Option<Uuid>,
    pub quantity: i32,
    pub action: String,
}

/// Order decomposer
pub struct OrderDecomposer;

impl OrderDecomposer {
    /// Decompose a product order into service and resource orders
    pub fn decompose(product_order: &ProductOrder) -> DecompositionResult {
        let mut service_orders = vec![];
        let mut resource_orders = vec![];
        let mut tasks = vec![];

        // For each product order item, create service orders
        if let Some(order_items) = &product_order.order_item {
            for order_item in order_items {
                let service_order_id = Uuid::new_v4();
                let service_order_spec = ServiceOrderSpec {
                    id: service_order_id,
                    product_order_item_id: order_item.id,
                    service_specification_id: order_item.product_offering.as_ref().map(|po| po.id),
                    quantity: order_item.quantity.unwrap_or(1),
                    action: order_item.action.clone(),
                };
                service_orders.push(service_order_spec.clone());

                // Create service order task
                let service_task = FulfillmentTask {
                    id: Uuid::new_v4(),
                    order_id: product_order.base.id,
                    task_type: TaskType::ServiceOrder(service_order_id),
                    state: crate::state::FulfillmentState::Acknowledged,
                    dependencies: vec![],
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    completed_at: None,
                    error: None,
                };
                tasks.push(service_task.clone());

                // For each service order, create resource orders
                // This is a simplified example - in reality, you'd look up service specifications
                // to determine required resources
                let resource_order_id = Uuid::new_v4();
                let resource_order_spec = ResourceOrderSpec {
                    id: resource_order_id,
                    service_order_id,
                    resource_specification_id: None,
                    quantity: service_order_spec.quantity,
                    action: service_order_spec.action.clone(),
                };
                resource_orders.push(resource_order_spec.clone());

                // Create resource order task with dependency on service order
                let resource_task = FulfillmentTask {
                    id: Uuid::new_v4(),
                    order_id: product_order.base.id,
                    task_type: TaskType::ResourceOrder(resource_order_id),
                    state: crate::state::FulfillmentState::Acknowledged,
                    dependencies: vec![service_task.id],
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    completed_at: None,
                    error: None,
                };
                tasks.push(resource_task);
            }
        }

        DecompositionResult {
            product_order_id: product_order.base.id,
            service_orders,
            resource_orders,
            tasks,
        }
    }
}
