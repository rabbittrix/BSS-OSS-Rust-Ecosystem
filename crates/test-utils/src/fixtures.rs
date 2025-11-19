//! Test fixtures and mock data

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

/// Create a test customer JSON
pub fn create_test_customer_json() -> serde_json::Value {
    json!({
        "name": "Test Customer",
        "description": "Test customer for unit tests",
        "status": "ACTIVE",
        "contactMedium": [
            {
                "mediumType": "EMAIL",
                "characteristic": {
                    "emailAddress": "test@example.com"
                }
            }
        ]
    })
}

/// Create a test product order JSON
pub fn create_test_product_order_json() -> serde_json::Value {
    json!({
        "name": "Test Product Order",
        "description": "Test order for unit tests",
        "orderItem": [
            {
                "id": Uuid::new_v4().to_string(),
                "action": "add",
                "quantity": 1,
                "state": "ACKNOWLEDGED"
            }
        ],
        "state": "ACKNOWLEDGED"
    })
}

/// Create a test product offering JSON
pub fn create_test_product_offering_json() -> serde_json::Value {
    json!({
        "name": "Test Product Offering",
        "description": "Test offering for unit tests",
        "isBundle": false,
        "lifecycleStatus": "ACTIVE",
        "validFor": {
            "startDateTime": Utc::now().to_rfc3339(),
            "endDateTime": (Utc::now() + chrono::Duration::days(365)).to_rfc3339()
        }
    })
}

/// Create test bill JSON
pub fn create_test_bill_json(customer_id: Uuid) -> serde_json::Value {
    json!({
        "name": "Test Bill",
        "description": "Test bill for unit tests",
        "billDate": Utc::now().to_rfc3339(),
        "billNo": format!("BILL-{}", Uuid::new_v4()),
        "relatedParty": [
            {
                "id": customer_id.to_string(),
                "role": "customer"
            }
        ],
        "totalAmount": {
            "amount": 100.0,
            "currency": "USD"
        }
    })
}

/// Create test usage record JSON
pub fn create_test_usage_record_json() -> serde_json::Value {
    json!({
        "name": "Test Usage Record",
        "description": "Test usage record for unit tests",
        "usageType": "DATA",
        "usageDate": Utc::now().to_rfc3339(),
        "quantity": {
            "amount": 1024.0,
            "units": "MB"
        }
    })
}
