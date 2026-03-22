use crate::config;
use crate::models::{AppConfig, CompilationResult, NasConfig, Order};
use crate::{compiler, nas_client, xml_parser};

/// Parse an XML order file and return the structured order data
#[tauri::command]
pub fn parse_xml_file(path: String) -> Result<Order, String> {
    xml_parser::parse_order_xml(&path).map_err(|e| e.to_string())
}

/// Run the full compilation process
#[tauri::command]
pub fn compile_order(
    app: tauri::AppHandle,
    xml_path: String,
    output_dir: String,
    nas_config: NasConfig,
) -> Result<CompilationResult, String> {
    let order = xml_parser::parse_order_xml(&xml_path).map_err(|e| e.to_string())?;
    compiler::compile_order(&app, &order, &output_dir, &nas_config).map_err(|e| e.to_string())
}

/// Load the saved application configuration
#[tauri::command]
pub fn load_config() -> Result<AppConfig, String> {
    config::load_config().map_err(|e| e.to_string())
}

/// Save the application configuration
#[tauri::command]
pub fn save_config(config: AppConfig) -> Result<(), String> {
    config::save_config(&config).map_err(|e| e.to_string())
}

/// Test connectivity to the NAS
#[tauri::command]
pub fn test_nas_connection(nas_config: NasConfig) -> Result<serde_json::Value, String> {
    match nas_client::test_connection(&nas_config) {
        Ok(msg) => Ok(serde_json::json!({ "success": true, "message": msg })),
        Err(e) => Ok(serde_json::json!({ "success": false, "message": e.to_string() })),
    }
}
