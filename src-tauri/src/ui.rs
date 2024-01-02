use std::path::PathBuf;
use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn show_win1(handle: &AppHandle) {
  let url = WindowUrl::App(PathBuf::from("/"));
  let builder = WindowBuilder::new(handle, "win1", url)
    .title("win-title1")
    .inner_size(600.into(), 400.into())
    .min_inner_size(600.into(), 400.into())
    .disable_file_drop_handler()
    .visible(false)
    .focused(true);

  #[cfg(target_os = "macos")]
  let builder = {
    use tauri::{menu::Menu, TitleBarStyle};
    builder
      .hidden_title(true)
      .title_bar_style(TitleBarStyle::Overlay)
      .menu(Menu::default(handle).unwrap_or(Menu::new(handle)))
  };

  let win = builder.build().unwrap();

  #[cfg(any(windows, target_os = "macos"))]
  window_shadows::set_shadow(&win, true).unwrap();
  #[cfg(target_os = "windows")]
  win.set_decorations(false).unwrap();

  win.show().unwrap();
  win.set_focus().unwrap();
}
