-- App database baseline.
-- Cross-document state for the whole installation. One app.db per machine.

-- Recently opened documents (most-recently-used list).
CREATE TABLE IF NOT EXISTS recent_files (
    id          TEXT PRIMARY KEY,           -- UUIDv7
    path        TEXT NOT NULL UNIQUE,        -- absolute path to the .appdoc file
    title       TEXT NOT NULL,
    last_opened DATETIME NOT NULL
);

-- Installation-wide key/value preferences.
CREATE TABLE IF NOT EXISTS preferences (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
