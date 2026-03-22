use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::models::AppConfig;

/// Get the config file path (~/.config/arsenal-filecompiler/config.json)
fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("arsenal-filecompiler");
    config_dir.join("config.json")
}

/// Load the application configuration from disk
pub fn load_config() -> Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Impossible de lire la configuration : {:?}", path))?;
    serde_json::from_str(&content).context("Format de configuration invalide")
}

/// Save the application configuration to disk
pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&path, content)
        .with_context(|| format!("Impossible de sauvegarder la configuration : {:?}", path))?;
    Ok(())
}
