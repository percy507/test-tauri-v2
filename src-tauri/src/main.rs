// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod menu;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmds::toggle_devtools])
    .plugin(tauri_plugin_window::init())
    .setup(|app| menu::init_menu(app.handle()))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
