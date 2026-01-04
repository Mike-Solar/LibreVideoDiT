mod camera;
mod config;
mod hash;
mod importer;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn import_sd_card(sd_card_path: String) -> Result<importer::ImportReport, String> {
    importer::import_sd_card(sd_card_path).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, import_sd_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn copy_files(_dest_path: String){
    
}
