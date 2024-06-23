use tauri::{AppHandle, Manager};

#[cfg(debug_assertions)]
#[tauri::command]
pub async fn toggle_devtools(handle: AppHandle, label: String) {
  if let Some(win) = handle.get_webview_window(&label) {
    if win.is_devtools_open() {
      win.close_devtools();
    } else {
      win.open_devtools();
    }
  }
}
