// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

mod cmds;
mod menu;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmds::toggle_devtools])
    .plugin(tauri_plugin_window::init())
    .on_window_event(|event| {
      let handle = event.window().app_handle();
      let label = event.window().label();
      match event.event() {
        WindowEvent::Focused(focused) => {
          println!("WindowEvent::Focused ==> {:?} {:?}", focused, label);
          if *focused {
            #[cfg(target_os = "macos")]
            match menu::setup_menu(handle, &label) {
              Ok(()) => {}
              Err(err) => println!("Failed to setup menu, {:?}", err),
            }
          }
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
