
mod services;
mod cmd;
use crate::cmd::{get_img_list, greet, open_url};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_img_list,
            open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
