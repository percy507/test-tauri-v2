use std::path::PathBuf;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

pub fn show_win_1(handle: &AppHandle) {
  let label = "win_1";
  let url = WebviewUrl::App(PathBuf::from("/"));
  let builder = WebviewWindowBuilder::new(handle, label, url)
    .title("win-title1")
    .inner_size(1000.into(), 800.into())
    .min_inner_size(1000.into(), 800.into())
    .disable_drag_drop_handler()
    .visible(false)
    .focused(true);

  #[cfg(target_os = "macos")]
  let builder = {
    use tauri::{menu::Menu, TitleBarStyle};
    builder
      .hidden_title(true)
      .title_bar_style(TitleBarStyle::Overlay)
      .menu(Menu::default(handle).unwrap())
  };

  let win = builder.build().unwrap();

  win.set_shadow(true).unwrap();
  #[cfg(target_os = "windows")]
  win.set_decorations(false).unwrap();

  win.show().unwrap();
  win.set_focus().unwrap();
}
