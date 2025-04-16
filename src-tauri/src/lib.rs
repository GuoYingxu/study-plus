use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

// 定义菜单

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let update_i = MenuItem::with_id(app, "update", "检查更新", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i, &update_i])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("Quit menu item clicked");
                        app.exit(0);
                    }
                    "update" => {
                        println!("Update menu item clicked");
                        // tauri_plugin_update::check_update(app, None).unwrap();
                    }
                    _ => {
                        println!("Unknown menu item clicked");
                    }
                })
                .build(app)
                .expect("Failed to create tray icon");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}
