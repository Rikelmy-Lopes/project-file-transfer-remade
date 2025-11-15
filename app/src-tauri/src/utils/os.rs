use local_ip_address::local_ip;

#[tauri::command]
pub fn get_current_ip() -> String {
    let ip = local_ip().unwrap().to_string();
    ip
}
