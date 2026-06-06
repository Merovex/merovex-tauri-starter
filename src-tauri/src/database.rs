//! Database layer: SQLite connections + Refinery migrations.
//!
//! Two databases:
//! - **app db** (`app.db` in the OS app-data dir): cross-document state such as
//!   recent files and preferences. One per installation.
//! - **document db** (a `.appdoc` file the user picks): one SQLite database *is*
//!   one user document. One per open document.
//!
//! Every connection is configured identically (foreign keys ON, WAL, busy
//! timeout) before use. Migrations are embedded into the binary at compile time
//! and run automatically when a database is opened for the first time.
//!
//! Connection strategy: **open → operate → close per request.** There is no
//! pool. `open_*_database()` runs migrations (call on first open / startup);
//! `open_*_conn()` skips the migration check for ordinary CRUD.

use rusqlite::Connection;
use std::path::Path;
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database error: {0}")]
    Rusqlite(#[from] rusqlite::Error),

    #[error("migration error: {0}")]
    Migration(#[from] refinery::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

// Migrations are embedded from these directories at compile time. Add new
// migrations as `V{n}__{description}.sql` files; they run in version order.
mod app_migrations {
    use refinery::embed_migrations;
    embed_migrations!("migrations/app_db");
}

mod document_migrations {
    use refinery::embed_migrations;
    embed_migrations!("migrations/document_db");
}

/// Apply the standard pragmas. Run on EVERY connection before use.
fn configure_connection(conn: &Connection) -> rusqlite::Result<()> {
    // Foreign keys are not persisted; they must be enabled per connection.
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    // WAL gives better read/write concurrency and crash safety.
    conn.execute_batch("PRAGMA journal_mode = WAL;")?;
    // Wait on a lock instead of failing immediately.
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    // Good safety/perf balance with WAL.
    conn.execute_batch("PRAGMA synchronous = NORMAL;")?;
    // 64 MB page cache (negative = KiB).
    conn.execute_batch("PRAGMA cache_size = -64000;")?;
    Ok(())
}

/// Open the app database, creating it and running migrations as needed.
pub fn open_app_database(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    debug!("opening app database at {:?}", path);
    let mut conn = Connection::open(path)?;
    configure_connection(&conn)?;
    app_migrations::migrations::runner().run(&mut conn)?;
    info!("app database ready at {:?}", path);
    Ok(conn)
}

/// Open a document database (a `.appdoc` file), creating it and running
/// migrations as needed.
pub fn open_document_database(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    debug!("opening document database at {:?}", path);
    let mut conn = Connection::open(path)?;
    configure_connection(&conn)?;
    document_migrations::migrations::runner().run(&mut conn)?;

    // Lightweight integrity check; log a warning if anything's off.
    if let Ok(result) = conn.query_row("PRAGMA quick_check", [], |r| r.get::<_, String>(0)) {
        if result != "ok" {
            warn!("integrity check on {:?}: {}", path, result);
        }
    }

    info!("document database ready at {:?}", path);
    Ok(conn)
}

/// Lightweight app-db opener for CRUD. Does NOT run migrations.
pub fn open_app_conn(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    configure_connection(&conn)?;
    Ok(conn)
}

/// Lightweight document-db opener for CRUD. Does NOT run migrations.
pub fn open_document_conn(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    configure_connection(&conn)?;
    Ok(conn)
}

/// Remove stale `-wal` / `-shm` sidecar files left after a crash.
/// Call before retrying an open that failed due to a lock.
#[allow(dead_code)] // utility kept for crash-recovery wiring
pub fn clear_stale_locks(path: &Path) -> std::io::Result<()> {
    let p = path.to_string_lossy();
    for suffix in ["-wal", "-shm"] {
        let sidecar = std::path::PathBuf::from(format!("{p}{suffix}"));
        if sidecar.exists() {
            std::fs::remove_file(&sidecar)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("starter-test-{}", uuid::Uuid::now_v7()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn fresh_document_database_has_tables_and_fk() {
        let dir = temp_dir();
        let path = dir.join("test.appdoc");
        let conn = open_document_database(&path).unwrap();

        let fk: i32 = conn.query_row("PRAGMA foreign_keys", [], |r| r.get(0)).unwrap();
        assert_eq!(fk, 1);

        let exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='items'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(exists, 1);
    }

    #[test]
    fn reopen_is_idempotent() {
        let dir = temp_dir();
        let path = dir.join("test.appdoc");
        drop(open_document_database(&path).unwrap());
        // Second open must not re-run or fail migrations.
        let _ = open_document_database(&path).unwrap();
    }
}
