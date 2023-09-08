use tauri::AppHandle;

#[cfg(debug_assertions)]
#[tauri::command]
pub async fn toggle_devtools(handle: AppHandle, label: String) {
  use tauri::Manager;

  if let Some(win) = handle.get_window(&label) {
    if win.is_devtools_open() {
      win.close_devtools();
    } else {
      win.open_devtools();
    }
  }
}
