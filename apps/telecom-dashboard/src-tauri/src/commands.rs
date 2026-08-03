use std::sync::Arc;
use tauri::State;
use telecom_product_engine::{
    BnplRequest, DataWalletTransferRequest, IdentityIssueRequest, InMemoryGateway, LogicStep,
    Money, ProductEvent, ProductOrchestrator, ProductService, TopUpRequest, TurboBoostRequest,
};
use uuid::Uuid;

pub struct AppState {
    pub engine: Arc<ProductOrchestrator<InMemoryGateway>>,
}

#[tauri::command]
pub async fn list_product_events(
    state: State<'_, AppState>,
    limit: usize,
) -> Result<Vec<ProductEvent>, String> {
    state
        .engine
        .recent_events(limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_logic_steps(
    state: State<'_, AppState>,
    limit: usize,
    flow_id: Option<String>,
) -> Result<Vec<LogicStep>, String> {
    let flow = flow_id.as_deref().and_then(|s| Uuid::parse_str(s).ok());
    state
        .engine
        .recent_logic_steps(limit, flow)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn real_time_topup(
    state: State<'_, AppState>,
    amount: f64,
    unit: String,
) -> Result<serde_json::Value, String> {
    let result = state
        .engine
        .real_time_topup(TopUpRequest {
            customer_id: Uuid::new_v4(),
            amount: Money { value: amount, unit },
            channel: "tauri-dashboard".into(),
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn turbo_boost(
    state: State<'_, AppState>,
    minutes: u32,
) -> Result<serde_json::Value, String> {
    let result = state
        .engine
        .turbo_boost(TurboBoostRequest {
            customer_id: Uuid::new_v4(),
            duration_minutes: minutes,
            slice_name: "turbo-embb".into(),
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn data_wallet_transfer(
    state: State<'_, AppState>,
    amount_gb: f64,
) -> Result<serde_json::Value, String> {
    let donor = Uuid::new_v4();
    let recipient = Uuid::new_v4();
    state
        .engine
        .seed_balance(
            donor,
            Money {
                value: amount_gb + 5.0,
                unit: "GB".into(),
            },
            "DATA",
        )
        .await
        .map_err(|e| e.to_string())?;

    let result = state
        .engine
        .data_wallet_transfer(DataWalletTransferRequest {
            donor_party_id: donor,
            recipient_party_id: recipient,
            amount: Money {
                value: amount_gb,
                unit: "GB".into(),
            },
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn bnpl_device(
    state: State<'_, AppState>,
    device_name: String,
    total: f64,
    installments: u32,
) -> Result<serde_json::Value, String> {
    let result = state
        .engine
        .bnpl_device(BnplRequest {
            party_id: Uuid::new_v4(),
            device_name,
            total_amount: Money {
                value: total,
                unit: "EUR".into(),
            },
            installments,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn issue_identity(
    state: State<'_, AppState>,
    login: String,
) -> Result<serde_json::Value, String> {
    let result = state
        .engine
        .issue_identity(IdentityIssueRequest {
            party_id: Uuid::new_v4(),
            login,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn list_catalog() -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![
        serde_json::json!({"id": "off-5g", "name": "5G Unlimited", "status": "ACTIVE"}),
        serde_json::json!({"id": "off-fiber", "name": "Fiber 1Gbps", "status": "ACTIVE"}),
        serde_json::json!({"id": "off-iot", "name": "IoT Starter", "status": "RETIRED"}),
    ])
}
