
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// #[tauri::command]
// fn parse_file(file_path: &str) -> String {
//     println!("你传入了文件路径:{} ",file_path);
//     return "解析成功".to_string();
// }

use std::fs;

#[tauri::command]
fn parse_file(file_path: &str) -> Result<String, String> {
    println!("你传入了文件路径:{} ",file_path);
    return fs::read_to_string(&file_path).map_err(|err|err.to_string());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,parse_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}