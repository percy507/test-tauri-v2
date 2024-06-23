// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target, TargetKind};

mod cmds;
mod ui;

fn main() {
  let mut app = tauri::Builder::default()
    .plugin(
      tauri_plugin_log::Builder::default()
        .targets([
          Target::new(TargetKind::LogDir {
            file_name: Some(String::from("main")),
          }),
          Target::new(TargetKind::Stderr),
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::Webview),
        ])
        .filter(|meta| !meta.target().contains("tao::"))
        .build(),
    )
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![cmds::toggle_devtools])
    .setup(|app| {
      ui::show_win_1(app.handle());
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Regular);

  app.run(|handle, event| match event {
    tauri::RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  });
}
