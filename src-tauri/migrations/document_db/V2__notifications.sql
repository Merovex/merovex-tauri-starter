-- Notifications inbox, scoped to THIS document/project (the .appdoc) — the
-- account-level inbox in Basecamp terms. It aggregates across the records inside
-- this database; it does NOT span other .appdoc files.
--
-- A notification can also be a REMINDER: set `remind_at` to have it resurface at
-- that time. It can point to a TARGET RECORD in this same database via
-- target_kind/target_id (intra-DB deep link — no path needed since it's local).
--
-- Derived states (computed in queries, no stored enum):
--   unread     → read_at IS NULL
--   scheduled  → remind_at IS NOT NULL AND remind_at > now      ("Scheduled" list)
--   due        → remind_at IS NOT NULL AND remind_at <= now     ("Reminders" top)
--   feed       → not dismissed and not a future reminder
--   dismissed  → dismissed_at IS NOT NULL

CREATE TABLE IF NOT EXISTS notifications (
    id           TEXT PRIMARY KEY,          -- UUIDv7
    kind         TEXT NOT NULL DEFAULT 'notification', -- message, comment, due, …
    title        TEXT NOT NULL,
    excerpt      TEXT,                        -- preview/snippet text
    icon         TEXT,                        -- sprite glyph name (e.g. 'megaphone-solid')
    actor        TEXT,                        -- who triggered it
    context      TEXT,                        -- where it lives (sub-project/team label)

    -- Deep link to the source record in THIS database (polymorphic; no FK).
    target_kind  TEXT,                        -- record type (e.g. 'item')
    target_id    TEXT,                        -- record id

    created_at   DATETIME NOT NULL,
    read_at      DATETIME,                    -- NULL = unread
    remind_at    DATETIME,                    -- NULL = not a reminder; set = resurface at this time
    reminded_at  DATETIME,                    -- last time it resurfaced (became "due")
    dismissed_at DATETIME                     -- soft-dismiss (removed from view)
);

CREATE INDEX IF NOT EXISTS idx_notifications_remind_at ON notifications(remind_at);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
