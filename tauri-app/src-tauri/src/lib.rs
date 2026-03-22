pub mod commands;
pub mod compiler;
pub mod config;
pub mod models;
pub mod nas_client;
pub mod xml_parser;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_xml_file,
            commands::compile_order,
            commands::load_config,
            commands::save_config,
            commands::test_nas_connection,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application");
}
