# CLAUDE.md

Guidance for Claude Code working in this repository.

## Project

A desktop app starter: **Tauri v2 (Rust) + Svelte 5 + Vite + SQLite (WAL)**.
Full design rationale is in `docs/ARCHITECTURE.md` — read it before non-trivial
changes.

## Critical rules

- **🚫 NEVER put CSS in a `.svelte` file. No exceptions.** No `<style>` blocks,
  ever. No static inline `style="…"` either. ALL styling lives in
  `src/styles/*.css`. The only tolerated inline use is passing a CSS custom
  property to a component (`style="--x: {value}"`) — that sets a variable, not
  styling. This is enforced: `pnpm build` runs `scripts/check-no-svelte-css.mjs`
  and FAILS the build on any `<style>` block. If you need a style, add a BEM
  class to `src/styles/components.css` (or a token/utility) and reference it.
- **Use `pnpm`** — never `npm` or `yarn`.
- **Do NOT run `pnpm tauri build` or `pnpm tauri dev`** unless asked — the user
  runs the app. Backend `cargo test` (in `src-tauri/`) is fine.
- **No CDN URLs.** All assets are local/bundled.
- **Migrations are forward-only.** Never edit a released `V{n}__*.sql`; add the
  next-numbered file instead.
- Run `cargo check` only after confirming with the user.

## Architecture in one screen

- **Data** (`src-tauri/src/database.rs`): two SQLite databases —
  installation-wide `app.db` (recent files, prefs) and per-document `.appdoc`
  files (a document *is* a database). Migrations via Refinery, embedded with
  `embed_migrations!`, run on first open. Connection model is **open → operate →
  close per request**; no pool.
- **Application** (`src-tauri/src/commands.rs`): every `#[tauri::command]` is the
  whole frontend surface. Register new ones in `lib.rs`. Map errors to `String`.
- **Presentation** (`src/`): Svelte 5 runes. ALL `invoke()` calls go through
  `src/lib/api.js`. Shared state lives in runes stores (`src/lib/stores/*.svelte.js`).

## Styling

**Read `docs/DESIGN.md` — it's the canonical design document.** A Basecamp 5–
inspired, single-surface system on **OpenProps** tokens; CSS is external-only
(see the critical rule above).

Files (all under `src/styles/`, imported by `index.css`):

- `tokens.css` — **the design's source of truth.** Two surfaces
  (`--color-page-tint` behind, `--color-canvas` on top), near-black `--color-ink`
  (with an `--ink-rgb` triplet for translucent overlays), three semantic accents
  in distinct lanes (`--color-blue` = interactive, `--color-green` = selected/
  affirmative, `--color-rust` = structural labels), state colors
  (`--color-danger`, `--color-warning`), seven page tints, the dark-mode swap,
  the elevation-ladder shadows, a `--z-*` scale, and motion/`--disabled-opacity`.
- `base.css` — reset + element defaults (body on the page tint, bold near-black
  headings, blue links).
- `utilities.css` — the concise set: `center`, `stack` (+`--tight`/`--loose`),
  `cluster` (+`--between`), `grid` (peer-card layout), `grow`, `plain-list`, and
  text roles `eyebrow` (rust), `muted`, `link`.
- `components.css` — the full BEM library: surfaces (`card`, `action-bar`), the
  elevation ladder (`menu`→`dialog`→`modal`), forms (`button` + variants,
  `field`/`select.field`/`textarea.field`, `checkbox`/`radio`/`switch`,
  `segmented`, `label`/`help`/`field-error`), display/feedback (`badge`,
  `banner`, `list`/`divider`, `spinner`, `empty-state`, `toast`,
  `[data-tooltip]`), and `table`/`tabs`/`accordion`/`avatar`/`icon`. **Check
  here (and `docs/DESIGN.md` §9) before building any new UI.**

Non-negotiables:
- **Reference semantic tokens only** (`var(--color-ink)`, `var(--size-5)`),
  never raw literals or OpenProps props directly in components.
- **Cards are peers** set off by padding + the faint `--shadow-halo` — never
  borders or elevation. They sit on `--color-canvas` above the page tint.
- **Reach down the elevation ladder by default.** Climb a tier (menu → dialog →
  modal) only when the surface's importance/blocking-ness justifies it.
- **Two re-tint axes via `<html>`:** `data-tint` (warm/green/purple/blue/red/
  grey) sets the page tint; `data-mode` (light/dark, absent = auto) sets the
  mode. Dark mode collapses all tints to one blue — expected, don't "fix" it.
- A **utility** earns its place only if repeated widely; else a **BEM class**.
- **Accessibility is baseline:** keep `:focus-visible` styling, gate new
  animations behind `prefers-reduced-motion`, use `.visually-hidden` for
  icon-only controls, and use the `--z-*` scale (never ad-hoc z-index).

### Icons

One `currentColor` sprite at `public/sprites.svg` (267 glyphs, 24×24, paired
outline/`-solid`). Always use `import Icon from '$lib/components/Icon.svelte'`
and `<Icon name="…" />` — never a hand-rolled `<svg>` or a one-off asset. Browse
names in `docs/SPRITE_INVENTORY.md`. Sizes are class-driven (`.icon`,
`.icon--sm` 14px, `.icon--lg` 24px).

## Conventions (match these)

- **IDs:** UUIDv7 — `Uuid::now_v7().simple().to_string()`.
- **Timestamps:** RFC 3339 strings; `created_at` + `updated_at` on every row.
- **Soft delete:** set `discarded_at`; read with `WHERE discarded_at IS NULL`.
  Reserve hard `DELETE` for permanent removal.
- **Row reads:** named columns — `row.get("title")`, never `row.get(0)`.
- **Pragmas:** never disable `foreign_keys`; it's set per connection in
  `configure_connection`.
- **Command args:** Rust `snake_case` params arrive from JS as `camelCase`
  (`doc_path` ↔ `{ docPath }`). Keep `api.js` in sync with `commands.rs`.

## Placeholders to replace

The `items` table, the `Item` model, its commands, and the demo UI in
`App.svelte` exist only to show the round trip. Delete them once the real domain
exists. Keep the `metadata` KV table.

## Renaming the document type (the `.appdoc` placeholder)

`.appdoc` is a placeholder (a SQLite DB under the hood, so it's cosmetic to the
data layer — `database.rs` opens whatever path it's given).

**Single source of truth: `src-tauri/tauri.conf.json` →
`bundle.fileAssociations[0].ext`.** Set it once and rebuild:
- **Rust** compiles it in via `build.rs` → `config::DOC_EXT` (used by the
  open-with handler in `lib.rs`).
- **Frontend** receives it via `vite.config.js` `define` → `src/lib/doc.js`
  (`DOC_EXT` / `DOC_FILTERS`: dialog filters, default filename, UI copy).

The only other spot to touch is **`.gitignore`** (the `*.appdoc*` line — dev
hygiene). Pick a unique, lowercase extension; no other code changes needed.

Treat one `.appdoc` as the **account/project (workspace)** — its records and its
notifications inbox live inside it (the `document_db` migration set).

## Adding a feature (the loop)

1. `migrations/document_db/V{n}__….sql` (or `app_db/`) for schema.
2. Struct in `src-tauri/src/models.rs`.
3. `#[tauri::command]` in `commands.rs` + register in `lib.rs`.
4. Wrapper in `src/lib/api.js`.
5. Drive it from a store/component.
