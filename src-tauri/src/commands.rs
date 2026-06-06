//! Tauri commands: the entire surface the frontend can call via `invoke()`.
//!
//! Pattern for every command:
//!   1. Resolve the database path (document path comes from the frontend; the
//!      app.db path is derived from Tauri's app-data dir).
//!   2. Open a connection (open → operate → close, no pooling).
//!   3. Do the work, map errors to String (Tauri requires Serialize errors).
//!
//! Document commands take `doc_path: String` so the frontend stays in control
//! of which document is active. The app db is resolved from the AppHandle.

use crate::database::{open_app_conn, open_app_database, open_document_conn, open_document_database};
use crate::models::{Item, RecentFile};
use chrono::Utc;
use rusqlite::params;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

fn new_id() -> String {
    Uuid::now_v7().simple().to_string()
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

/// Path to the installation-wide app.db inside the OS app-data directory.
/// Generic over runtime so the menu builder can reuse it.
pub fn app_db_path<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("could not resolve app data dir: {e}"))?;
    Ok(dir.join("app.db"))
}

/// App name/version/build info for the About dialog.
#[derive(serde::Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub build_date: String,
    pub doc_ext: String,
}

#[tauri::command]
pub fn app_info(app: AppHandle) -> AppInfo {
    let pkg = app.package_info();
    AppInfo {
        name: pkg.name.clone(),
        version: pkg.version.to_string(),
        build_date: crate::config::BUILD_DATE.to_string(),
        doc_ext: crate::config::DOC_EXT.to_string(),
    }
}

// ===========================================================================
// Document lifecycle
// ===========================================================================

/// Create a new (or open an existing) document at `doc_path`, running migrations.
/// Records it in the recent-files list. Call this when the user picks a file.
/// Best-effort daily snapshot of a document into the app-data backups folder.
/// One copy per file per day; checkpoints the WAL first so the file is current.
fn daily_backup<R: tauri::Runtime>(app: &AppHandle<R>, doc_path: &str) {
    let Ok(base) = app.path().app_data_dir() else {
        return;
    };
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let dir = base.join("backups").join(&date);
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let src = Path::new(doc_path);
    let Some(name) = src.file_name() else {
        return;
    };
    let dest = dir.join(name);
    if dest.exists() {
        return; // already snapshotted today
    }
    if let Ok(conn) = crate::database::open_document_conn(src) {
        let _ = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
    }
    let _ = std::fs::copy(src, &dest);
}

/// Optimize a document: checkpoint the WAL and VACUUM.
#[tauri::command]
pub fn optimize_document(doc_path: String) -> Result<(), String> {
    let conn = open_document_conn(Path::new(&doc_path)).map_err(|e| e.to_string())?;
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); VACUUM;")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn open_document(app: AppHandle, doc_path: String) -> Result<(), String> {
    let path = Path::new(&doc_path);
    open_document_database(path).map_err(|e| e.to_string())?;
    daily_backup(&app, &doc_path);

    // Record in recent files (app.db). Best-effort; don't fail the open over it.
    if let Ok(app_path) = app_db_path(&app) {
        let _ = open_app_database(&app_path); // ensure migrated
        if let Ok(conn) = open_app_conn(&app_path) {
            let title = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".into());
            let _ = conn.execute(
                "INSERT INTO recent_files (id, path, title, last_opened)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(path) DO UPDATE SET title = excluded.title,
                                                 last_opened = excluded.last_opened",
                params![new_id(), doc_path, title, now()],
            );
        }
    }
    Ok(())
}

#[tauri::command]
pub fn list_recent(app: AppHandle) -> Result<Vec<RecentFile>, String> {
    let app_path = app_db_path(&app)?;
    open_app_database(&app_path).map_err(|e| e.to_string())?;
    let conn = open_app_conn(&app_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, path, title, last_opened
             FROM recent_files ORDER BY last_opened DESC LIMIT 20",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(RecentFile {
                id: row.get("id")?,
                path: row.get("path")?,
                title: row.get("title")?,
                last_opened: row.get("last_opened")?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())
}

// ===========================================================================
// Items (PLACEHOLDER domain — replace with your own commands)
// ===========================================================================

#[tauri::command]
pub fn list_items(doc_path: String) -> Result<Vec<Item>, String> {
    let conn = open_document_conn(Path::new(&doc_path)).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, body, position, created_at, updated_at
             FROM items WHERE discarded_at IS NULL
             ORDER BY position, created_at",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get("id")?,
                title: row.get("title")?,
                body: row.get("body")?,
                position: row.get("position")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_item(doc_path: String, title: String) -> Result<Item, String> {
    let conn = open_document_conn(Path::new(&doc_path)).map_err(|e| e.to_string())?;
    let id = new_id();
    let ts = now();
    conn.execute(
        "INSERT INTO items (id, title, body, position, created_at, updated_at)
         VALUES (?1, ?2, '', 0, ?3, ?3)",
        params![id, title, ts],
    )
    .map_err(|e| e.to_string())?;
    Ok(Item {
        id,
        title,
        body: String::new(),
        position: 0,
        created_at: ts.clone(),
        updated_at: ts,
    })
}

#[tauri::command]
pub fn update_item(
    doc_path: String,
    id: String,
    title: String,
    body: String,
) -> Result<(), String> {
    let conn = open_document_conn(Path::new(&doc_path)).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE items SET title = ?2, body = ?3, updated_at = ?4 WHERE id = ?1",
        params![id, title, body, now()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Soft delete (sets discarded_at). Use a real DELETE only for permanent removal.
#[tauri::command]
pub fn delete_item(doc_path: String, id: String) -> Result<(), String> {
    let conn = open_document_conn(Path::new(&doc_path)).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE items SET discarded_at = ?2 WHERE id = ?1",
        params![id, now()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
