use tauri::{
  menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
  AppHandle,
};

pub fn init_menu(handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  let pkg_info = handle.package_info();
  let app_name = &pkg_info.name.clone();

  let app_menu = SubmenuBuilder::new(handle, "App")
    .item(
      &MenuItemBuilder::with_id("id_aboutApp", format!("About {}", app_name))
        .build(handle),
    )
    .separator()
    .item(
      &MenuItemBuilder::with_id("id_preferences", "Settings")
        .accelerator("command+,")
        .build(handle),
    )
    .separator()
    .services()
    .separator()
    .hide()
    .hide_others()
    .show_all()
    .separator()
    .quit();

  let test_menu = SubmenuBuilder::new(handle, "Test")
    .item(&MenuItemBuilder::with_id("toggle", "Toggle").build(handle))
    .item(&CheckMenuItemBuilder::new("Mark").build(handle));

  let menu = MenuBuilder::new(handle)
    .item(&app_menu.build()?)
    .item(&test_menu.build()?)
    .build()?;

  handle.set_menu(menu)?;

  handle
    .on_menu_event(move |handle, event| println!("event id: {:?}", event.id));
  Ok(())
}
