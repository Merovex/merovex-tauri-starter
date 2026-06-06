-- Document database baseline (one .appdoc file == one document).
--
-- This schema is a PLACEHOLDER to demonstrate the round trip. Replace `items`
-- with your real domain tables. Keep `metadata` — it's a handy per-document
-- key/value store (schema version notes, document title, settings, etc.).
--
-- Conventions worth keeping:
--   * Text primary keys holding UUIDv7 (time-ordered, sortable, merge-friendly).
--   * created_at / updated_at on every row.
--   * Soft delete via discarded_at (NULL = live) instead of DELETE.
--   * Foreign keys with ON DELETE/UPDATE as appropriate (FK enforcement is ON).

-- Per-document key/value store.
CREATE TABLE IF NOT EXISTS metadata (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- PLACEHOLDER domain table. Replace with your own.
CREATE TABLE IF NOT EXISTS items (
    id           TEXT PRIMARY KEY,           -- UUIDv7
    title        TEXT NOT NULL,
    body         TEXT NOT NULL DEFAULT '',
    position     INTEGER NOT NULL DEFAULT 0,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    discarded_at DATETIME                     -- NULL = live, set = soft-deleted
);

CREATE INDEX IF NOT EXISTS idx_items_position ON items(position);
