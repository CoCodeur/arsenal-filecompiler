use anyhow::{Context, Result};
use log::{error, info};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Emitter};

use crate::models::{CompilationProgress, CompilationResult, NasConfig, Order};
use crate::nas_client::NasConnection;

/// Run the full compilation process:
/// 1. Create folder hierarchy (order_id/material_type/)
/// 2. Connect to NAS via SMB
/// 3. Copy matching PDFs into the correct material folders
/// 4. Report missing files
pub fn compile_order(
    app: &AppHandle,
    order: &Order,
    output_dir: &str,
    nas_config: &NasConfig,
) -> Result<CompilationResult> {
    let total = order.products.len() as u32;
    let mut files_copied: u32 = 0;
    let mut files_missing: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // Step 1: Create output directory structure
    let order_path = Path::new(output_dir).join(&order.id);
    fs::create_dir_all(&order_path)
        .with_context(|| format!("Impossible de créer le dossier : {:?}", order_path))?;

    emit_progress(app, 0, total, "Création de l'arborescence...", "processing");

    // Collect unique materials and create folders
    let materials: HashSet<&str> = order.products.iter().map(|p| p.material.as_str()).collect();
    for material in &materials {
        let mat_path = order_path.join(sanitize_folder_name(material));
        fs::create_dir_all(&mat_path)?;
    }

    // Step 2: Connect to NAS
    emit_progress(app, 0, total, "Connexion au NAS...", "processing");
    let nas = match NasConnection::connect(nas_config) {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Erreur de connexion NAS : {}", e);
            error!("{}", msg);
            return Ok(CompilationResult {
                success: false,
                files_copied: 0,
                files_missing: Vec::new(),
                errors: vec![msg],
                output_path: order_path.to_string_lossy().to_string(),
            });
        }
    };

    // Step 3: Process each product
    for (i, product) in order.products.iter().enumerate() {
        let progress_msg = format!(
            "Traitement de {} ({}/{})",
            product.reference,
            i + 1,
            total
        );
        emit_progress(app, i as u32, total, &progress_msg, "processing");

        let material_folder = order_path.join(sanitize_folder_name(&product.material));

        // Try to find the PDF on the NAS
        match nas.find_pdf(&product.category, &product.reference) {
            Ok(Some(remote_path)) => {
                // Copy the file (handle quantity > 1)
                for q in 0..product.quantity {
                    let filename = if product.quantity == 1 {
                        format!("{}.pdf", product.reference)
                    } else {
                        format!("{}_{}.pdf", product.reference, q + 1)
                    };

                    let local_path = material_folder.join(&filename);
                    match nas.copy_to_local(&remote_path, &local_path) {
                        Ok(()) => {
                            files_copied += 1;
                            info!("Copié : {} -> {:?}", remote_path, local_path);
                        }
                        Err(e) => {
                            let msg = format!("Erreur copie {} : {}", product.reference, e);
                            error!("{}", msg);
                            errors.push(msg);
                        }
                    }
                }
            }
            Ok(None) => {
                let msg = format!(
                    "{} (catégorie: {})",
                    product.reference, product.category
                );
                files_missing.push(msg);
            }
            Err(e) => {
                let msg = format!("Erreur recherche {} : {}", product.reference, e);
                errors.push(msg);
            }
        }
    }

    // Step 4: Generate log file for missing files
    if !files_missing.is_empty() {
        let log_path = order_path.join("fichiers_manquants.log");
        let log_content = files_missing.join("\n");
        fs::write(&log_path, &log_content)
            .unwrap_or_else(|e| error!("Impossible d'écrire le log : {}", e));
    }

    let success = errors.is_empty();
    emit_progress(
        app,
        total,
        total,
        if success {
            "Compilation terminée !"
        } else {
            "Compilation terminée avec des erreurs"
        },
        if success { "success" } else { "error" },
    );

    Ok(CompilationResult {
        success,
        files_copied,
        files_missing,
        errors,
        output_path: order_path.to_string_lossy().to_string(),
    })
}

/// Sanitize a string for use as a folder name
fn sanitize_folder_name(name: &str) -> String {
    name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
        .trim()
        .to_string()
}

/// Emit a progress event to the frontend
fn emit_progress(app: &AppHandle, current: u32, total: u32, message: &str, status: &str) {
    let _ = app.emit(
        "compilation-progress",
        CompilationProgress {
            current,
            total,
            message: message.to_string(),
            status: status.to_string(),
        },
    );
}
