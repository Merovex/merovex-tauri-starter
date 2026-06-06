//! Tauri application entry point: plugins, native menu, logging, open-with
//! handling, and the command table. `main.rs` just calls `run()`.

mod commands;
mod config;
mod database;
mod logging;
mod menu;
mod models;

use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // Desktop-only plugins. single-instance MUST be registered first: on a second
    // launch it forwards argv to the running instance instead of starting anew.
    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.set_focus();
                }
                if let Some(path) = config::doc_path_from_args(args) {
                    let _ = app.emit("open-document", path);
                }
            }))
            .plugin(tauri_plugin_window_state::Builder::default().build())
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_process::init());
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // File + stdout logging; keep the guard alive in managed state.
            if let Ok(dir) = app.path().app_log_dir() {
                let guard = logging::init(&dir);
                app.manage(guard);
            }

            #[cfg(desktop)]
            {
                let menu = menu::build(app.handle())?;
                app.set_menu(menu)?;

                // System tray with a small menu.
                use tauri::menu::{MenuBuilder, MenuItemBuilder};
                use tauri::tray::TrayIconBuilder;
                let show = MenuItemBuilder::with_id("tray-show", "Show").build(app)?;
                let quit = MenuItemBuilder::with_id("tray-quit", "Quit").build(app)?;
                let tray_menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;
                let mut tray = TrayIconBuilder::new().menu(&tray_menu).on_menu_event(
                    |app, event| match event.id().as_ref() {
                        "tray-show" => {
                            if let Some(w) = app.get_webview_window("main") {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                        "tray-quit" => app.exit(0),
                        _ => {}
                    },
                );
                if let Some(icon) = app.default_window_icon().cloned() {
                    tray = tray.icon(icon);
                }
                tray.build(app)?;
            }

            // Document passed on first launch (Windows/Linux "open with").
            if let Some(path) =
                config::doc_path_from_args(std::env::args().skip(1).collect::<Vec<_>>())
            {
                let _ = app.emit("open-document", path);
            }
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "new" => {
                let _ = app.emit("menu:new", ());
            }
            "open" => {
                let _ = app.emit("menu:open", ());
            }
            "preferences" => {
                let _ = app.emit("menu:preferences", ());
            }
            "about" => {
                let _ = app.emit("menu:about", ());
            }
            "reveal-logs" => {
                if let Ok(dir) = app.path().app_log_dir() {
                    use tauri_plugin_opener::OpenerExt;
                    let _ = app
                        .opener()
                        .open_path(dir.to_string_lossy().to_string(), None::<&str>);
                }
            }
            "reveal-backups" => {
                if let Ok(dir) = app.path().app_data_dir() {
                    use tauri_plugin_opener::OpenerExt;
                    let backups = dir.join("backups");
                    let _ = std::fs::create_dir_all(&backups);
                    let _ = app
                        .opener()
                        .open_path(backups.to_string_lossy().to_string(), None::<&str>);
                }
            }
            "close-doc" => {
                let _ = app.emit("menu:close-doc", ());
            }
            "check-update" => {
                let _ = app.emit("menu:check-update", ());
            }
            other => {
                if let Some(path) = other.strip_prefix("recent:") {
                    let _ = app.emit("open-document", path.to_string());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::optimize_document,
            commands::open_document,
            commands::list_recent,
            commands::list_items,
            commands::create_item,
            commands::update_item,
            commands::delete_item,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            let _ = (&app, &event);
            // macOS delivers "open with" as an Apple event, surfaced here.
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    if let Ok(path) = url.to_file_path() {
                        let _ = app.emit("open-document", path.to_string_lossy().to_string());
                    }
                }
            }
        });
}
