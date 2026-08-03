// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::AppState;
use std::sync::Arc;
use tauri::Emitter;
use telecom_product_engine::{InMemoryGateway, ProductOrchestrator};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(ProductOrchestrator::new(InMemoryGateway::default()));
    let state = AppState {
        engine: engine.clone(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(move |app| {
            let handle = app.handle().clone();
            let mut rx = engine.subscribe_logic();
            tauri::async_runtime::spawn(async move {
                while let Ok(step) = rx.recv().await {
                    let _ = handle.emit("logic-step", step);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_product_events,
            commands::list_logic_steps,
            commands::real_time_topup,
            commands::turbo_boost,
            commands::data_wallet_transfer,
            commands::bnpl_device,
            commands::issue_identity,
            commands::list_catalog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
