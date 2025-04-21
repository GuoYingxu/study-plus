use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

use std::fs::{ File};
use std::io::BufReader;
use std::path::Path;
use tauri::command;
use zip::ZipArchive;

#[command]
fn extract_zip(file_path: String, extract_to: String) -> Result<(), String> {
    let file_path = std::fs::canonicalize(file_path)
        .map_err(|e| format!("Failed to canonicalize file path: {}", e))?;
    let extract_to = std::fs::canonicalize(extract_to)
        .map_err(|e| format!("Failed to canonicalize extract path: {}", e))?;
    print!("file_path: {:?}", file_path);
    print!("extract_to: {:?}", extract_to);
    let file = File::open(&file_path).map_err(|e| format!("Failed to open zip file: {}", e))?;
    let mut archive = ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;
        let out_path = Path::new(&extract_to).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            let mut outfile =
                File::create(&out_path).map_err(|e| format!("Failed to create file: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }

    Ok(())
}

// 定义菜单

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
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
        .invoke_handler(tauri::generate_handler![
            extract_zip,
            // 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}
