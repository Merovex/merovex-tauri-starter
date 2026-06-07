# Features (as built)

What this starter ships with today. Items marked **(placeholder)** are demo
scaffolding to replace; **(inert)** needs configuration before it does anything.

## Stack

- **Tauri v2** (Rust backend) + **Svelte 5** (runes) + **Vite 7**.
- **SQLite** via `rusqlite` (bundled) with **Refinery** versioned migrations.
- pnpm-only; no CDN assets.

## Data layer

- **Two databases:** installation-wide `app.db` (recent files, preferences) and
  per-document `.appdoc` files (one SQLite DB = one workspace/project).
- **Versioned migrations** (`V{n}__name.sql`, embedded, forward-only) run on
  open, tracked in `refinery_schema_history`.
- **Per-connection pragmas:** `foreign_keys=ON`, WAL, busy timeout, `NORMAL`
  sync, 64 MB cache. Open → operate → close (no pool).
- **Conventions:** UUIDv7 text PKs, `created_at`/`updated_at`, soft delete via
  `discarded_at`, named-column reads.
- **Integrity check** (`PRAGMA quick_check`) on document open (logs warnings).
- **Optimize on close** (`wal_checkpoint(TRUNCATE)` + `VACUUM`).
- **Daily backups** of the open document to `<app-data>/backups/<date>/`.
- Commands: `app_info`, `open_document`, `list_recent`, `list_items`,
  `create/update/delete_item`, `optimize_document`. **(`items` is placeholder)**

## Document model

- One file = one SQLite database with a custom extension (**`.appdoc`
  placeholder**).
- **Single source of truth** for the extension: `tauri.conf.json`
  → `bundle.fileAssociations[0].ext`. `build.rs` compiles it into Rust
  (`config::DOC_EXT`); `vite.config.js` injects it to the frontend
  (`src/lib/doc.js`).
- Window title reflects the open document: `App Name: Document`.

## Desktop integration

- **Native menu** (App/File/Edit/Window/Help) with accelerators: `⌘N` new,
  `⌘O` open, `⌘W` close, `⌘,` settings.
- **Open with / double-click** a document + **single-instance** (focuses the
  running app, forwards the path; macOS via `RunEvent::Opened`).
- **Window-state persistence** (size/position/maximized).
- **Open Recent** menu, built from `app.db`.
- **About** dialog (name, version, build date, document type).
- **File logging** (daily rolling, OS log dir) + Help → **Reveal Logs**.
- **System tray** (Show / Quit).
- **Command palette** (`⌘K`): New, Open, Close, Preferences, Notifications,
  About, Check for updates.
- **Drag-and-drop** an `.appdoc` onto the window to open it.
- **Confirm-on-destructive** deletes (toggle in Settings).
- Help → **Reveal Backups**.
- **Auto-update** scaffolding (`updater` + `process` plugins, menu/palette
  entry). **(inert** until you set `plugins.updater` endpoints + signing key.)**

## Design system (Basecamp 5–inspired)

See [`DESIGN.md`](./DESIGN.md). Highlights:

- **Single-surface model:** two surface tokens, re-tintable page (7 tints via
  the "drop"), dark mode as a token swap (tints collapse), restrained semantic
  palette (blue interactive / green selected / rust labels + danger/warning).
- **OpenProps** tokens; **all CSS external** — no `<style>` in `.svelte`
  (build-enforced by `scripts/check-no-svelte-css.mjs`).
- **Four-tier elevation ladder:** card → menu → dialog → modal (with the modal
  spec measured exactly).
- **Component library:** card, button (+variants), field / skinned `select`,
  segmented control, badge, banner, list/divider, spinner, empty-state, toast,
  tooltip, table, tabs, accordion, avatar, icon; concise layout utilities; full
  token cheat-sheet.
- **Accessibility baseline:** `:focus-visible`, `prefers-reduced-motion`,
  `:disabled`, `.visually-hidden`, a `--z-*` scale.
- **Icons:** one 267-glyph `currentColor` SVG sprite + `<Icon>` component
  ([`SPRITE_INVENTORY.md`](./SPRITE_INVENTORY.md)).
- Light/dark + theme persisted to localStorage.

## Notifications & reminders

See [`NOTIFICATIONS.md`](./NOTIFICATIONS.md). UI is complete; the feed is a
sample store **(placeholder)** over real schema (`document_db/V2`).

- Right-side **push panel** (fixed width, animates the main area aside).
- **New for you** / **Previous notifications**, with mark read/unread.
- **Reminders** (due) surface at the top; **Scheduled** sub-view (back arrow,
  date pills).
- Per-row **Remind me later** (Now / Tomorrow / Next weekend / Next week / Pick
  a date with a native calendar) and **Show now / Cancel reminder**.
- Footer toggle reflects state (count + "New for you" / "Notifications" / ✕).
- Filter; empty state.

## Developer experience

- **No-CSS-in-`.svelte`** guard wired into `pnpm build`.
- **Prettier** + `.editorconfig` + `rustfmt.toml` + `format` / `format:check`
  scripts.
- **Placeholder icon generator** (`scripts/make-placeholder-icon.mjs`).
- Docs: ARCHITECTURE, DESIGN, NOTIFICATIONS, SPRITE_INVENTORY, this file.

## Not included (deliberate)

Crash reporting and first-run onboarding were intentionally skipped. Settings are
localStorage-backed (no `app.db`-backed prefs). Notifications aren't wired to the
DB yet (see NOTIFICATIONS.md → "What's wired vs. not").
