# Notifications & Reminders

The notifications panel (`src/lib/components/Notifications.svelte`, footer toggle
in `Footer.svelte`) is a Basecamp-style inbox with a built-in **Remind** system
(our rename of "Bubble Up" — see history). This documents the data model and
what's left to make it real.

## Data model (document-scoped)

Notifications are scoped to the open document/project (the `.appdoc`) — the
account-level inbox in Basecamp terms. They aggregate across the records inside
*that* database and do NOT span other `.appdoc` files, so they live in the
**document database** — `migrations/document_db/V2__notifications.sql`. (`app.db`
stays for truly machine-global state: recent files, preferences, license.)

Key columns:

| Column | Purpose |
| --- | --- |
| `title`, `excerpt`, `icon`, `actor`, `context` | what to render in a row |
| `target_kind` / `target_id` | **deep link** to a record in this same DB |
| `created_at` | feed ordering |
| `read_at` | NULL = unread (drives the "1" badge + footer count) |
| `remind_at` | NULL = plain notification; set = a **reminder** that resurfaces then |
| `reminded_at` | last time it became "due" |
| `dismissed_at` | soft-dismiss |

### Derived states (no stored enum — compute in queries)

| UI section | Rule |
| --- | --- |
| **New for you** (unread feed) | `read_at IS NULL AND dismissed_at IS NULL AND (remind_at IS NULL OR remind_at <= now)` |
| **Previous notifications** | read, not dismissed, not a future reminder |
| **Reminders** (top, due) | `remind_at <= now AND dismissed_at IS NULL` |
| **Scheduled** | `remind_at > now AND dismissed_at IS NULL` |

These map 1:1 to the current frontend getters (`due`, `scheduled`, unread feed).

## Target records (the deep link)

Each notification can point at the record that produced it. Basecamp renders this
as an `<a href="…/messages/9969714818">` wrapping the row. Since the inbox is
document-scoped, the target lives in the **same** database — store `target_kind`
+ `target_id` (no path needed). Clicking a row navigates to that record — the
wiring point for "click notification → jump to the thing."

## Reminders (`remind_at`)

- "Remind me later" / "Pick a date" sets `remind_at`.
- A reminder with `remind_at` in the **future** shows under **Scheduled**.
- When `remind_at` passes it becomes **due** and rises to the top under
  **Reminders** (set `reminded_at`).
- "Show now" clears `remind_at` (back into the feed as unread); "Cancel reminder"
  sets `dismissed_at` (or deletes).
- **Resurfacing needs a trigger:** check on app launch + on panel open, and/or a
  lightweight interval timer, flipping scheduled → due when `remind_at <= now`.
  (For cross-restart accuracy, compute from `remind_at` on open rather than
  relying on a live timer.)

## Date picker

The starter uses the **native `<input type="date">`** for "Pick a date" — its
built-in calendar, zero dependencies, accessible. Basecamp uses the **Duet date
picker** (Duet Design System — a framework-agnostic, accessible web component:
`@duetds/date-picker`). For a custom in-app calendar matching our design tokens,
either:
- adopt Duet (vendor it locally — no CDN), or
- build a small Svelte `DatePicker.svelte` styled to the tokens (dark selected
  cell, MO–SU headers, month/year nav).

The native input is fine until a themed calendar is wanted.

## What's wired vs. not

- ✅ Full UI: feed, New/Previous, Reminders (due), Scheduled view, per-row remind
  menu (Now/Tomorrow/…/Pick a date), Show now / Cancel reminder, mark read/unread,
  filter, footer toggle + push panel.
- ✅ Document-DB schema (`document_db/V2`) with `remind_at` + target columns.
- ⬜ **Backend wiring:** the frontend still reads an in-memory sample store
  (`src/lib/stores/notifications.svelte.js`). To go real, add Tauri commands
  (`list_notifications`, `mark_read`, `set_remind`, `dismiss`, `show_now`) — each
  taking `doc_path` like the other document commands — over the `notifications`
  table, and point the store at them via `src/lib/api.js`.
- ⬜ **Deep-link navigation** from a row to its `target_*` record.
- ⬜ **Resurfacing trigger** (scheduled → due).
