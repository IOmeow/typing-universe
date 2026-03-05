mod state;
mod control;

use tauri::{Emitter, Manager, Window, menu::{MenuBuilder, MenuItemBuilder}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}};
use std::{thread};
use rdev::{listen, EventType};
use state::AppState;

#[tauri::command]
fn show_window(window: Window) {
    window.show().unwrap();
    window.maximize().unwrap();
    window.set_ignore_cursor_events(true).unwrap();
}

#[tauri::command]
fn hide_control(window: Window) {
    window.hide().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            show_window,
            hide_control,
            control::update_settings,
            control::get_settings,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            thread::spawn(move || {
                if let Err(error) = listen(move |event| {
                    match event.event_type {
                        EventType::KeyPress(key) => {
                            let key_str = format!("{:?}", key);
                            let _ = app_handle.emit("global-key", key_str);
                            return;
                        }
                        EventType::MouseMove { x, y } => {
                            let _ = app_handle.emit("mouse-move", (x as f32, y as f32));
                        }
                        _ => {}
                    }
                }) {
                    println!("Error: {:?}", error);
                }
            });

            // ========= 建立選單項目 =========
            let toggle_fullscreen =
                MenuItemBuilder::new("切換全螢幕").id("toggle_fullscreen").build(app)?;

            let toggle_visibility =
                MenuItemBuilder::new("顯示 / 隱藏視窗").id("toggle_visibility").build(app)?;

            let toggle_control =
                MenuItemBuilder::new("控制").id("toggle_control").build(app)?;

            let quit =
                MenuItemBuilder::new("離開").id("quit").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[
                    &toggle_fullscreen,
                    &toggle_visibility,
                    &toggle_control,
                    &quit,
                ])
                .build()?;

            // ========= 建立 Tray Icon =========
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    let window = app.get_webview_window("main").unwrap();
                    let control = app.get_webview_window("control").unwrap();

                    match event.id().as_ref() {
                        "toggle_fullscreen" => {
                            if window.is_maximized().unwrap() {
                                window.set_size(tauri::PhysicalSize{width: 200, height: 200}).unwrap();
                                window.set_ignore_cursor_events(false).unwrap();
                            }
                            else {
                                window.maximize().unwrap();
                                window.set_ignore_cursor_events(true).unwrap();
                            }
                        }
                        "toggle_visibility" => {
                            if window.is_visible().unwrap() {
                                window.hide().unwrap();
                            } else {
                                window.show().unwrap();
                            }
                        }
                        "toggle_control" => {
                            if control.is_visible().unwrap() {
                                control.hide().unwrap();
                            } else {
                                control.show().unwrap();
                            }
                        }
                        "quit" => {
                            window.close().unwrap();
                            control.close().unwrap();
                            // app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                    } => {
                    println!("left click pressed and released");
                    // in this example, let's show and focus the main window when the tray is clicked
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                    }
                    }
                    _ => {}
                })
                .build(app)?;

            control::load_store(&app.handle(), &app.state::<AppState>());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
