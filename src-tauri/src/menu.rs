use tauri::{
  menu::{
    CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem,
    SubmenuBuilder,
  },
  AppHandle,
};

pub fn setup_menu(
  handle: &AppHandle,
  label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let pkg_info = handle.package_info();
  let app_name = &pkg_info.name.clone();

  let app_menu = SubmenuBuilder::new(handle, app_name).quit().build()?;

  let native_menu = SubmenuBuilder::new(handle, "NativeMenu")
    .items(&[
      &PredefinedMenuItem::about(handle, Some("关于"), None),
      &PredefinedMenuItem::services(handle, Some("服务")),
      &PredefinedMenuItem::hide(handle, Some("隐藏")),
      &PredefinedMenuItem::hide_others(handle, Some("隐藏其他")),
      &PredefinedMenuItem::show_all(handle, Some("显示全部")),
      &PredefinedMenuItem::quit(handle, Some("退出")),
      &PredefinedMenuItem::close_window(handle, Some("关闭窗口")),
      &PredefinedMenuItem::undo(handle, Some("撤销")),
      &PredefinedMenuItem::redo(handle, Some("重做")),
      &PredefinedMenuItem::cut(handle, Some("剪切")),
      &PredefinedMenuItem::copy(handle, Some("拷贝")),
      &PredefinedMenuItem::paste(handle, Some("粘贴")),
      &PredefinedMenuItem::select_all(handle, Some("全选")),
      &PredefinedMenuItem::fullscreen(handle, Some("切换全屏")),
      &PredefinedMenuItem::minimize(handle, Some("最小化")),
      &PredefinedMenuItem::maximize(handle, Some("最大化")),
    ])
    .build()?;

  // only for "win_1"
  let custom_menu = SubmenuBuilder::new(handle, "Test")
    .item(&MenuItemBuilder::with_id("toggle", "Toggle").build(handle))
    .separator()
    .item(&CheckMenuItemBuilder::new("Mark").build(handle))
    .build()?;

  let mut menus = MenuBuilder::new(handle).item(&app_menu).item(&native_menu);

  if label == "win1" {
    menus = menus.item(&custom_menu);
  }

  handle.set_menu(menus.build()?)?;
  handle.on_menu_event(move |handle, event| {
    println!("event id: {:?}", event.id);
  });

  Ok(())
}
