use serde::{Deserialize, Serialize};

/// A product entry from an XML order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub category: String,
    pub reference: String,
    pub description: String,
    pub material: String,
    pub quantity: u32,
}

/// A parsed order from XML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub client: String,
    pub address: String,
    pub date: String,
    pub products: Vec<Product>,
}

/// NAS connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasConfig {
    pub server: String,
    pub share_path: String,
    pub username: String,
    pub password: String,
}

/// Full application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub nas: NasConfig,
    pub default_output_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            nas: NasConfig {
                server: String::new(),
                share_path: String::new(),
                username: String::new(),
                password: String::new(),
            },
            default_output_dir: String::new(),
        }
    }
}

/// Progress event sent to the frontend during compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationProgress {
    pub current: u32,
    pub total: u32,
    pub message: String,
    pub status: String, // "processing", "success", "error"
}

/// Result of a compilation run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub success: bool,
    pub files_copied: u32,
    pub files_missing: Vec<String>,
    pub errors: Vec<String>,
    pub output_path: String,
}
