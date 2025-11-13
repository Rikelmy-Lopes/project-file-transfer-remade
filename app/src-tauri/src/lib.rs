mod fs;
mod http;
mod utils;
use http::server::{get_current_ip, start_server};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_server, get_current_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
