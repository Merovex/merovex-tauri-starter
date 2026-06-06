//! Native application menu. Custom items emit `menu:*` events the frontend
//! handles; standard roles use Tauri's predefined items. Open Recent is built
//! at startup from the app.db `recent_files` table (refreshes on relaunch).

use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Runtime};

pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let new = MenuItemBuilder::with_id("new", "New")
        .accelerator("CmdOrCtrl+N")
        .build(app)?;
    let open = MenuItemBuilder::with_id("open", "Open…")
        .accelerator("CmdOrCtrl+O")
        .build(app)?;
    let close_doc = MenuItemBuilder::with_id("close-doc", "Close")
        .accelerator("CmdOrCtrl+W")
        .build(app)?;
    let prefs = MenuItemBuilder::with_id("preferences", "Settings…")
        .accelerator("CmdOrCtrl+,")
        .build(app)?;
    let about = MenuItemBuilder::with_id("about", "About").build(app)?;
    let reveal_logs = MenuItemBuilder::with_id("reveal-logs", "Reveal Logs").build(app)?;
    let reveal_backups = MenuItemBuilder::with_id("reveal-backups", "Reveal Backups").build(app)?;
    let check_update =
        MenuItemBuilder::with_id("check-update", "Check for Updates…").build(app)?;

    // Open Recent
    let mut recent_builder = SubmenuBuilder::new(app, "Open Recent");
    let recents = recent_files(app);
    if recents.is_empty() {
        let none = MenuItemBuilder::with_id("recent-none", "No recent files")
            .enabled(false)
            .build(app)?;
        recent_builder = recent_builder.item(&none);
    } else {
        for (path, title) in &recents {
            let item =
                MenuItemBuilder::with_id(format!("recent:{path}"), title.as_str()).build(app)?;
            recent_builder = recent_builder.item(&item);
        }
    }
    let open_recent = recent_builder.build()?;

    let mut menu = MenuBuilder::new(app);

    // macOS app menu (leading) holds About / Settings / Quit by convention.
    #[cfg(target_os = "macos")]
    {
        let app_menu = SubmenuBuilder::new(app, app.package_info().name.clone())
            .item(&about)
            .separator()
            .item(&prefs)
            .separator()
            .quit()
            .build()?;
        menu = menu.item(&app_menu);
    }

    // File
    let mut file = SubmenuBuilder::new(app, "File")
        .item(&new)
        .item(&open)
        .item(&open_recent);
    #[cfg(not(target_os = "macos"))]
    {
        file = file.separator().item(&prefs);
    }
    file = file.separator().item(&close_doc);
    #[cfg(not(target_os = "macos"))]
    {
        file = file.quit();
    }
    let file = file.build()?;
    menu = menu.item(&file);

    // Edit (standard roles)
    let edit = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;
    menu = menu.item(&edit);

    // Window
    let window = SubmenuBuilder::new(app, "Window").minimize().build()?;
    menu = menu.item(&window);

    // Help (About lives here off macOS)
    #[cfg(target_os = "macos")]
    let help = SubmenuBuilder::new(app, "Help")
        .item(&reveal_logs)
        .item(&reveal_backups)
        .separator()
        .item(&check_update)
        .build()?;
    #[cfg(not(target_os = "macos"))]
    let help = SubmenuBuilder::new(app, "Help")
        .item(&about)
        .item(&reveal_logs)
        .item(&reveal_backups)
        .separator()
        .item(&check_update)
        .build()?;
    menu = menu.item(&help);

    menu.build()
}

fn recent_files<R: Runtime>(app: &AppHandle<R>) -> Vec<(String, String)> {
    let Ok(path) = crate::commands::app_db_path(app) else {
        return vec![];
    };
    let _ = crate::database::open_app_database(&path); // ensure migrated
    let Ok(conn) = crate::database::open_app_conn(&path) else {
        return vec![];
    };
    let mut out = vec![];
    if let Ok(mut stmt) =
        conn.prepare("SELECT path, title FROM recent_files ORDER BY last_opened DESC LIMIT 10")
    {
        if let Ok(rows) = stmt.query_map([], |r| {
            Ok((r.get::<_, String>("path")?, r.get::<_, String>("title")?))
        }) {
            for row in rows.flatten() {
                out.push(row);
            }
        }
    }
    out
}
