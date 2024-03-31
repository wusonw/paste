// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Wry,
};
use tauri_plugin_store::{with_store, StoreCollection};

fn main() {
    let clear = CustomMenuItem::new("clear".to_string(), "Clear Clipboard Logs");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(clear)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "clear" => {
                    let stores = app.state::<StoreCollection<Wry>>();
                    let path = PathBuf::from(".clip-store.dat");
                    let _ = with_store(app.clone(), stores, path, |store| store.clear());
                    let clip_window = app
                        .get_window("clipboard")
                        .expect("Can not find clipboard window");
                    let _ = clip_window.emit("custom://reload-clip-records", ());
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
