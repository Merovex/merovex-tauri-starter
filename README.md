# Tauri Starter

A minimal desktop-app starter: **Tauri v2 (Rust) + Svelte 5 + Vite + SQLite**.
It demonstrates three architectural patterns worth keeping:

1. **Versioned SQL migrations** via [Refinery], embedded into the binary.
2. **SQLite-as-document**: one `.appdoc` file *is* one SQLite database, plus a
   separate installation-wide `app.db`.
3. **Svelte 5 frontend** talking to Rust through Tauri **commands** (`invoke`).

The `items` table and its commands are a **placeholder** to show the round trip.
Replace them with your real domain. See `docs/ARCHITECTURE.md` for the full tour.

The UI is a **Basecamp 5–inspired single-surface system** (two surface tokens,
re-tintable page, dark mode that collapses tints, a rationed four-tier elevation
ladder) on **OpenProps** tokens, with one `currentColor` icon sprite. The full
design is in [`docs/DESIGN.md`](docs/DESIGN.md); the 267-icon set in
[`docs/SPRITE_INVENTORY.md`](docs/SPRITE_INVENTORY.md). **All CSS is external**
in `src/styles/` — `.svelte` files contain no styling, and `pnpm build` fails if
they do (`scripts/check-no-svelte-css.mjs`).

## Features

Full list in [`docs/FEATURES.md`](docs/FEATURES.md). Highlights:

- **Data:** dual SQLite databases (`app.db` + per-document `.appdoc`), Refinery
  versioned migrations, UUIDv7 + soft deletes, integrity check on open, optimize
  on close, daily backups.
- **Document model:** one file = one SQLite DB; extension is single-sourced from
  `tauri.conf.json` into both Rust and the frontend; window title shows the doc.
- **Desktop:** native menu + shortcuts, open-with / single-instance,
  window-state, Open Recent, About, file logging, system tray, command palette
  (`⌘K`), drag-and-drop open, confirm-on-delete, auto-update scaffolding (inert
  until configured).
- **Design system:** Basecamp 5–inspired single-surface UI — re-tintable page (7
  tints), dark mode, four-tier elevation ladder, a full component library on
  OpenProps tokens, a 267-glyph `currentColor` icon sprite, all CSS external.
- **Notifications & reminders:** push panel with due/scheduled reminders,
  per-item snooze ("Remind me later") with a date picker, filter, footer badge.
- **DX:** no-CSS-in-`.svelte` guard, Prettier/rustfmt config, placeholder icon
  generator.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [pnpm](https://pnpm.io/) — this project uses **pnpm**, not npm/yarn
- Tauri system deps for your OS: https://tauri.app/start/prerequisites/

## First-time setup

```bash
pnpm install
```

**Add app icons** (required before the first build/dev run). Tauri generates all
formats from one square PNG:

```bash
pnpm tauri icon path/to/your-icon.png   # writes src-tauri/icons/*
```

## Make it your app

This is a starter — a few things are deliberately placeholders. When you spin up
a real app, set these first:

### 1. Define your document type

Each saved file is one SQLite database with a custom extension. `.appdoc` is a
**placeholder**; treat one document as a **workspace / project** (its records
*and* its notifications inbox live inside it).

**Set it once** in `src-tauri/tauri.conf.json` →
`bundle.fileAssociations[0].ext`. Rust compiles it in (`build.rs` →
`config::DOC_EXT`) and the frontend derives it (`vite.config.js` →
`src/lib/doc.js`), so the OS association, the open/save dialogs, and the
open-with handler all follow on the next build. The only other spot is the
`*.appdoc*` line in `.gitignore`.

Then design the document's schema in `src-tauri/migrations/document_db/` (and
installation-wide state in `migrations/app_db/`). See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

### 2. Set product identity

In **`src-tauri/tauri.conf.json`**: `productName`, `identifier`
(`com.you.yourapp`), and the window `title`. In **`package.json`**: `name`.

### 3. Replace the placeholder domain

The `items` table, `Item` model, its commands, and the demo UI in `App.svelte`
exist only to show the round trip — delete them once your real domain exists
(keep the `metadata` KV table). The notifications inbox is real schema
(`document_db/V2`); see [`docs/NOTIFICATIONS.md`](docs/NOTIFICATIONS.md).

## Desktop integration

Built in: native menu + shortcuts, open-with / single-instance, window-state,
Open Recent, About, file logging, system tray, drag-and-drop open, command
palette (`⌘K`), confirm-on-delete, daily document backups, and DB optimize.

- **Logs** live in the OS log dir (Help → Reveal Logs). **Backups** are daily
  copies in `<app-data>/backups/<date>/` (Help → Reveal Backups).
- **Auto-update is scaffolded but inert** until you configure it: add
  `plugins.updater` (`endpoints` + `pubkey`) to `tauri.conf.json` and sign your
  builds (`tauri signer generate`). Until then "Check for Updates…" reports a
  friendly error.

## Develop

```bash
pnpm tauri dev      # builds the frontend, launches the app with hot reload
```

## Build

```bash
pnpm tauri build    # produces a native installer for the current OS
```

## Run backend tests

```bash
cd src-tauri && cargo test
```

## Layout

```
tauri-starter/
├── src/                      # Svelte 5 frontend (Vite root)
│   ├── index.html
│   ├── main.js               # mounts App.svelte, imports styles
│   ├── App.svelte            # NO <style> / inline CSS — enforced
│   ├── styles/               # ALL CSS lives here (OpenProps + tokens)
│   │   ├── index.css         # entry: imports the rest in order
│   │   ├── tokens.css        # semantic roles + the single re-tintable canvas
│   │   ├── base.css          # reset + element defaults
│   │   ├── utilities.css     # concise utility set (stack/cluster/grid/…)
│   │   └── components.css    # BEM component classes (card/button/field)
│   └── lib/
│       ├── api.js            # ALL invoke() calls live here
│       ├── components/Icon.svelte      # sprite icon component
│       └── stores/document.svelte.js   # reactive state (runes)
├── public/sprites.svg        # the 267-icon currentColor sprite
├── docs/                     # FEATURES · ARCHITECTURE · DESIGN · NOTIFICATIONS · SPRITE_INVENTORY
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs           # entry → lib::run()
│   │   ├── lib.rs            # plugins + command registration
│   │   ├── database.rs       # connections + migration runners
│   │   ├── commands.rs       # #[tauri::command] surface
│   │   └── models.rs         # serde shapes shared with the frontend
│   ├── migrations/
│   │   ├── app_db/           # V1__…  (installation-wide)
│   │   └── document_db/      # V1__…  (per .appdoc document)
│   ├── capabilities/default.json
│   └── tauri.conf.json
├── vite.config.js
├── svelte.config.js
├── CLAUDE.md                 # guidance for Claude Code
└── docs/ARCHITECTURE.md
```

[Refinery]: https://crates.io/crates/refinery
