# Architecture

Three layers, one direction of dependency:

```
┌─ Presentation  Svelte 5 (runes) + Vite          src/
│        │  invoke() ↕ (and Tauri events, if you add them)
├─ Application   Rust + #[tauri::command]          src-tauri/src/commands.rs
│        │
└─ Data          SQLite (WAL) + Refinery           src-tauri/src/database.rs
                 app.db (global) + *.appdoc (per document)
```

---

## 1. Data layer — migrations and the two databases

### Two databases, two roles

| Database     | Where it lives                        | Holds                              |
| ------------ | ------------------------------------- | ---------------------------------- |
| `app.db`     | OS app-data dir (one per install)     | recent files, preferences          |
| `*.appdoc`   | wherever the user saves it            | one document's entire contents     |

A document file *is* a SQLite database with a custom extension. The user opens,
saves, and moves it like any file; we get transactions, indexes, and SQL for
free instead of inventing a file format.

### Migrations (Refinery)

SQL files live in `migrations/app_db/` and `migrations/document_db/`, named with
the Flyway convention **`V{n}__{description}.sql`**. The number drives ordering;
migrations are **forward-only**.

They are embedded into the binary at compile time:

```rust
mod document_migrations {
    use refinery::embed_migrations;
    embed_migrations!("migrations/document_db");
}
```

and run when a database is opened for the first time:

```rust
document_migrations::migrations::runner().run(&mut conn)?;
```

Refinery records applied migrations in a `refinery_schema_history` table inside
each database, so running again is a no-op.

**To change the schema:** add the next-numbered file (e.g.
`V2__add_tags.sql`). Never edit an already-released migration — write a new one.
A document created last year migrates itself forward the next time it's opened.

### Connection rules (`database.rs`)

- **Every** connection is configured before use: `foreign_keys = ON` (not
  persisted — must be set per connection), `journal_mode = WAL`,
  `busy_timeout = 5s`, `synchronous = NORMAL`, 64 MB cache.
- **Two tiers:** `open_*_database()` runs migrations (first open / startup);
  `open_*_conn()` skips them for ordinary CRUD.
- **Open → operate → close per request.** No connection pool, no shared mutable
  state. Each command opens what it needs and drops it. Simple and correct;
  revisit only if profiling shows it matters.
- WAL leaves `-wal`/`-shm` sidecars; `clear_stale_locks()` removes them after a
  crash before retrying an open.

### Data conventions

- **UUIDv7** text primary keys (`Uuid::now_v7().simple()`): time-ordered, so they
  sort chronologically and are friendlier to merge than autoincrement ints.
- `created_at` / `updated_at` on every row (RFC 3339 strings).
- **Soft delete** via `discarded_at` (NULL = live) instead of `DELETE`; filter
  `WHERE discarded_at IS NULL` in reads.
- **Named column access** when reading rows — `row.get("title")`, never
  `row.get(0)`. Index access silently breaks when columns are reordered.

---

## 2. Application layer — Tauri commands

`commands.rs` is the *entire* surface the frontend can reach. Each command:

1. Resolves a database path. Document commands take `doc_path: String` from the
   frontend (it owns "which document is active"); the `app.db` path is derived
   from `app.path().app_data_dir()`.
2. Opens a connection (`open_*_conn`).
3. Does the work and maps errors to `String` (Tauri requires a serializable
   error; `.map_err(|e| e.to_string())`).

Register every command in `lib.rs` via `tauri::generate_handler![…]`. Shapes
returned to JS live in `models.rs` and derive `Serialize`/`Deserialize`.

> Argument names cross the boundary as **camelCase**. A Rust parameter
> `doc_path` is passed from JS as `{ docPath }`. Tauri converts automatically;
> just keep the two spellings consistent (see `src/lib/api.js`).

---

## 3. Presentation layer — Svelte 5

- **Runes** for state: `$state`, `$derived`, `$effect` (enabled in
  `svelte.config.js`).
- **One place for `invoke()`:** `src/lib/api.js`. The rest of the UI imports
  named functions and never sees a command-name string. This keeps the
  Rust↔JS contract in a single auditable file.
- **Reactive state in runes stores** (`src/lib/stores/*.svelte.js`): a class
  instance with `$state` fields, exported as a singleton. Components read its
  fields reactively and mutate through its methods, which call `api.js` and keep
  local state in sync.

### Styling: external CSS, one canvas

**No CSS ever lives in a `.svelte` file** — no `<style>` blocks, no static
inline styles. This is enforced by `scripts/check-no-svelte-css.mjs`, which
`pnpm build` runs and fails on. Svelte gives free style scoping via `<style>`;
we deliberately trade that away for a single, auditable, greppable system. The
cost (global names) is paid with BEM; the win is one place to look and a rule
with no gray area — which matters when working with Claude.

The system lives in `src/styles/` (entry: `index.css`) and is built on
**OpenProps** tokens:

```
OpenProps props  →  tokens.css (Basecamp 5 model)  →  base.css
                 →  utilities.css (concise primitives)  →  components.css (BEM ladder)
```

It's a **Basecamp 5–inspired single-surface system**: two surface tokens
(`--color-page-tint` behind, `--color-canvas` on top), hierarchy from typography
+ space, a re-tintable page (`data-tint`), a dark mode that collapses all tints
to one blue (`data-mode`), and a four-tier elevation ladder (card → menu →
dialog → modal) where depth is rationed by importance. Icons are one
`currentColor` sprite (`public/sprites.svg`) via `<Icon>`.

**The full design is documented in [`DESIGN.md`](./DESIGN.md); the icon set in
[`SPRITE_INVENTORY.md`](./SPRITE_INVENTORY.md).** Components reference semantic
tokens (`var(--color-ink)`), never raw values.

### Choosing a communication channel

As the app grows you'll want more than `invoke`. Pick by intent:

| Need                                   | Use                                   |
| -------------------------------------- | ------------------------------------- |
| Call the backend / get data           | `invoke()` via `api.js`               |
| Shared state components observe        | a runes store (`*.svelte.js`)         |
| Native menu / OS event from Rust       | Tauri event (`listen('menu:…')`)      |
| One-off signal between components      | DOM `CustomEvent`                     |

This starter only needs the first two. Add the others when a real case appears
rather than up front.

---

## Extending the starter

1. **New domain table:** add `migrations/document_db/V2__….sql`.
2. **New model:** add a struct to `models.rs`.
3. **New operations:** add `#[tauri::command]` fns to `commands.rs`, register
   them in `lib.rs`.
4. **Expose to UI:** add wrappers to `src/lib/api.js`, then drive them from a
   store or component.
5. **Delete the placeholders:** remove the `items` table, `Item` model, its
   commands, and the demo UI in `App.svelte` once your own domain exists.
