# Design system

A Basecamp 5–inspired visual system: a calm, documentary, **single-surface**
design where flatness is the default, hierarchy comes from typography and
generous space, color is semantic and re-tintable from one token set, and
physical depth is rationed as a signal of importance.

All of this is token-driven (`src/styles/tokens.css`) and asset-light (one
`currentColor` SVG sprite). **No CSS lives in `.svelte` files** — see CLAUDE.md.

---

## 1. Two surfaces, one canvas feel

Almost everything resolves to two surface tokens:

| Token                | Role                          | Light        | Dark            |
| -------------------- | ----------------------------- | ------------ | --------------- |
| `--color-page-tint`  | the page behind               | ≈98% L tint  | `#0B151B`       |
| `--color-canvas`     | the card/region on top        | 100% L white | `#1B2930`       |

In **light** mode the two sit ≈98% vs 100% lightness — so close they're nearly
indistinguishable, which is what produces the famously flat feel. In **dark**
mode the gap widens (7.5% vs 14.7% L), so cards read as genuinely raised. The
flat look is partly a light-mode artifact of two near-identical near-whites.

Separation between surfaces is done with **hairlines and a faint ambient halo**,
never borders or drop shadows:

- Content cards: a soft `0 0 30px rgb(0 0 0 / 0.08)` bloom (`--shadow-halo`),
  **no border**.
- Region edges (e.g. the action bar): a single hairline underneath via
  `box-shadow: var(--color-hairline) 0 1px 0 0` — the same canvas color marked
  off by one line, not a differently-colored layer.

---

## 2. Layout: information at the top, navigation at the edges

- Each screen leads with **identity and context**: a small rust eyebrow label, a
  large bold near-black title, a one-line description — then content.
- **Almost no persistent chrome.** No left sidebar, no toolbars. Global
  navigation lives at the extreme top (a centered wordmark that launches the nav
  dialog) and a thin bottom utility row. The middle is all content.
- Information architecture is **broad and shallow**: a flat field of
  peer-level cards (`.grid`) you click *into*, not nested panels you drill
  *through*.

---

## 3. Color: semantic and re-tintable

Hierarchy is carried by type, space, and a **restrained semantic palette** — not
depth or decoration.

| Token              | Value (light)        | Job                              |
| ------------------ | -------------------- | -------------------------------- |
| `--color-ink`      | `rgb(43 55 62)`      | near-black headings + body       |
| `--color-ink-muted`| ink @ 60%            | secondary text                   |
| `--color-blue`     | `rgb(35 119 210)`    | interactive: links, actions, menus |
| `--color-green`    | `rgb(25 135 77)`     | selected / affirmative state     |
| `--color-danger`   | `rgb(204 51 51)`     | destructive actions / errors     |
| `--color-warning`  | `rgb(204 133 20)`    | caution                          |
| `--color-rust`     | rust                 | structural: section-label eyebrows |
| `--color-hairline` | `rgb(227 229 229)`   | 1px separators                   |

(`info` reuses `--color-blue`, `success` reuses `--color-green` — no separate
tokens.)

The three accents have distinct *jobs*, not just distinct hues: **blue** =
interactive (links, primary buttons, the menu surface); **green** = "which option
is currently true" (selected/confirmed state — never a call to action); **rust**
= structural section labels. Keep them in their lanes.

No gradients, no glassmorphism, no heavy shadows. Translucent **ink overlays**
(not new colors) do interaction states: `--overlay-hover` (ink @ 8%, button
hover), `--overlay-active` (ink @ 25%, menu-item highlight).

### The drop switcher — seven page tints

A single control re-tints the whole page by setting `data-tint` on `<html>`. All
slots are pinned ~96–98% L, so contrast never shifts:

| `data-tint` | Value                  | Hue                      |
| ----------- | ---------------------- | ------------------------ |
| `warm` (default) | `hsl(26 94% 98%)`  | warm cream/orange        |
| `green`     | `hsl(135 29% 97%)`     | green                    |
| `purple`    | `hsl(277 100% 98.5%)`  | purple                   |
| `blue`      | `hsl(214 100% 98%)`    | blue                     |
| `red`       | `hsl(0 82% 97.5%)`     | red/pink                 |
| `grey`      | `hsl(0 0% 96.5%)`      | neutral grey             |

(Basecamp defines a 7th slot, `tint-3`, identical to the default warm.) This is
only trivial *because* there's a single layer to re-color.

### Dark mode — a token swap, and the tints collapse

Dark mode keeps the exact same variable architecture; only values change
(`prefers-color-scheme`, overridable with `data-mode` on `<html>`). Crucially,
**the seven page-tints collapse to one dark blue** — the user's tint choice stops
mattering in dark mode, trading playful per-theme color for one consistent,
legible dark surface. Ink inverts to near-white (`hsl(205.7 13.2% 89.6%)`).

---

## 4. The elevation ladder — depth is rationed

Depth, shadow weight, corner radius, and whether the page gets blocked all
**increase together with importance**. Flatness is the default everywhere else.

| Tier | Class           | Surface                     | Radius | Shadow                | Blocks page? |
| ---- | --------------- | --------------------------- | ------ | --------------------- | ------------ |
| 0    | `.card`         | canvas on page tint         | 8px    | faint halo only       | no           |
| 1    | `.menu`         | solid blue, white text      | 8px    | none (color + z-20)   | no           |
| 2    | `.dialog`       | canvas/white                | 16px   | 5-step layered ramp   | no (no scrim)|
| 3    | `.modal` + `__panel` | canvas panel + dark scrim | 12px | 3-step + `z-index 300` | yes      |

- **Tier 1 (menu):** the `…` popover. Fully-saturated blue (`--color-blue`),
  white text, opens with a springy overshoot (`--ease-spring`), items highlight
  via an ink bar. On top by *color and stacking*, not simulated height.
- **Tier 2 (dialog):** the global nav/search surface. The one richly-elevated
  *floating* panel (doubled radius, real layered `--shadow-dialog`) — but **no
  scrim**, so it floats without blocking.
- **Tier 3 (modal):** the strongest "stop and deal with this" signal; reserve
  for focused tasks (preferences, blocking forms). Exact spec (measured live):
  - The `.modal` host **is** the scrim — no separate overlay child:
    `fixed; inset: 0; z-index: 300; background: rgb(0 0 0 / 0.5)`, a centering
    flex with `16px` padding. The scrim is **instant** (no fade).
  - `.modal__panel`: hard `width: 500px` (+ `max-width: 100%`, so the host's
    16px padding is the only responsive cap), `32px` padding, `12px` radius,
    `max-height: 100%; overflow-y: auto`, canvas background.
  - Shadow is a directional 3-step (`--shadow-modal`): a `1px` hairline ring
    plus two offset ambients — heavier than the dialog's even 5-step ramp.
  - Entrance: `modal-appear 100ms ease-in-out` — a quick scale-pop
    `0.85 → 1.02 → 1` with fade. **Deliberately distinct** from the menu's
    longer, bouncier `300ms` `--ease-spring`. Don't unify the two motions.

---

## 5. Icons — one sprite, tinted by inheritance

A single in-house SVG sprite at `public/sprites.svg`: **267 glyphs**, all
`24×24`, `fill: currentColor`, with paired outline/`-solid` weights and
Basecamp-specific symbols (`boost`, `answer`, `bubble-pop`, `hill-chart`,
`lineup`, `drop`). Because they inherit text color, they recolor flawlessly
across every theme and mode with no duplicate assets. Full list:
[`SPRITE_INVENTORY.md`](./SPRITE_INVENTORY.md).

Use the `Icon` component (never inline `<svg>`):

```svelte
<Icon name="bell" />            <!-- inherits color + 1.25rem default -->
<Icon name="check-solid" class="icon--sm" />   <!-- 14px, the menu default -->
<Icon name="trash" label="Delete" />           <!-- labelled = role=img -->
```

Sizes are class-driven: `.icon` (20px) · `.icon--sm` (14px) · `.icon--lg`
(24px, native).

> Note: the live Basecamp sprite is served hashed for cache-busting
> (`sprites-<hash>.svg`). We ship it locally at the stable `/sprites.svg`; the
> hash isn't needed for a bundled desktop app.

---

## 6. Fields & form controls

Inputs and `<select>`s share `.field`. Selects are **native** elements with
`appearance: none` and a CSS skin — the OS still renders the open option list;
only the closed control is styled.

- **Resting:** `40px` high, full-width, canvas background, `16px`/400 ink text,
  `8px` radius (the shared radius family), `padding: 0 12px`, and a `1px` border
  in **ink @ 15%** (`--color-field-border`) — a tinted hairline, not flat grey,
  so it cools/warms with the theme. No box-shadow.
- **Focus:** the OS outline is suppressed; the focus ring *is* a blue border
  (`--color-blue`) plus a tight `0 0 2px 0` blue bloom, faded in over `125ms`
  (only `box-shadow` transitions). Same blue as links/menus.
- **Select arrow:** a self-contained **double-chevron** (up over down = "cycles
  values") inlined as a data-URI in `--select-arrow` — no CDN — positioned
  `calc(100% - 12.8px) 50%` at `8.8px`. It's swapped to near-white in dark mode.
  Note this is distinct from the sprite's single `chevron-down`.

### Segmented control (`.segmented`)

A radio-style appearance control: a flex row of three **icon-over-label tiles**
separated by an `8px` gap — independent tiles, *not* a unified pill/track.

- **Tile:** `<button>`, column layout, icon (24px sprite, `currentColor`) over a
  `16px`/400 label; `76px` tall, `10.9px 13.6px` padding, `8px` radius, canvas
  background, `1px` border in **ink @ 25%** (`--color-field-border-strong` —
  heavier than the selects' 15%, for more tile definition). `transition: all`.
- **Hover (unselected):** background lifts to `--color-canvas-hover` (a very
  faint grey); border unchanged. Fill-only feedback.
- **Selected:** solid `--color-green` fill, border the same green, content flips
  to white (icon rides `currentColor`). No checkmark, ring, or shadow — the green
  fill alone signals state.
- **Accessibility:** drive it as a radiogroup (`role="radio"` + `aria-checked`);
  the selected style keys off `[aria-checked="true"]`. See `App.svelte`.

## 7. Rules of thumb

- Reference **semantic tokens only** in components (`var(--color-ink)`), never
  raw literals or OpenProps props directly.
- **Cards are peers** — halo + padding, never borders or elevation.
- A **utility** earns its place only if repeated widely; otherwise a **BEM
  component class**.
- Reach **down** the elevation ladder by default; climb a tier only when the
  surface's importance/blocking-ness justifies it.
- New icon? It's already in the sprite — just `<Icon name="…">`. Don't add
  one-off SVGs.

## 8. States & accessibility

- **Motion:** all animations (`menu-in`, `modal-appear`, `spin`) and the
  tint/mode transitions are neutralized under `@media (prefers-reduced-motion:
  reduce)` in `base.css`. New animations must survive that media query.
- **Focus:** a `2px` blue `:focus-visible` outline globally; fields/switches
  suppress it for their own blue ring. Never remove focus styling without a
  replacement.
- **Disabled:** `:disabled` / `[aria-disabled="true"]` → `--disabled-opacity`
  (0.5) + `not-allowed`. Wired for buttons and native inputs.
- **Hidden labels:** use `.visually-hidden` for icon-only controls (keeps them
  in the accessibility tree). Don't rely on `title`.
- **Placeholder / selection:** themed via `::placeholder` and `::selection`.
- **Z-index:** use the scale — `--z-sticky` 10, `--z-menu` 20, `--z-dialog` 100,
  `--z-modal` 300, `--z-toast` 400. No ad-hoc values.

## 9. Component library

All token-driven; pick the closest primitive before inventing markup.

**Buttons** — base `.button`; modifiers `--primary` (blue), `--secondary`
(outline), `--danger` (red), `--ghost` (blue text), `--icon` (square), sizes
`--sm`/`--lg`; `:disabled` handled; `.button-group` to join them.

**Forms** — `.form-group` (label + control + help), `.form-row` (two columns),
`.label`, `.required`, `.help`, `.field-error`, `.field--invalid`. Controls:
`.field` (input/`select`), `textarea.field` (auto-height), `.checkbox`/`.radio`
(blue accent), `.switch` (label + `__input` + `__track`, green "on"), and the
`.segmented` control (§6).

**Badges** — `.badge` (+ `--success`/`--info`/`--danger`): pill, tinted bg via
`color-mix`, for status like the green "All-access" marker.

**Banners / callouts** — `.banner` (+ `--info`/`--success`/`--warning`/
`--danger`) with `.banner__icon` + `.banner__body`. Inline notices; use the
matching sprite icon (`alert-circle`, `check-circle`, …).

**Lists & dividers** — `hr` / `.divider` (hairline); `.list` + `.list__row`
(hairline-separated rows, the non-card list).

**Feedback** — `.spinner` (loading), `.empty-state` (icon + message), `.toast`
via the `toast` store + `<Toaster>` (transient; `--success`/`--danger`),
`[data-tooltip="…"]` (pure-CSS tooltip on hover/focus).

**Data & navigation** — `.table` (hairline rows, uppercase muted headers);
`.tabs` + `.tab` (drive `aria-selected`); `.accordion` (native `<details>` +
`.accordion__body`, zero JS).

**Identity** — `<Avatar name="…" src="…">` → `.avatar` (+ `--sm`/`--lg`,
initials fallback).

Components that need behavior ship as Svelte files (`Toaster`, `Avatar`);
everything else is CSS classes + a documented markup pattern. Tabs, tooltip, and
accordion are intentionally JS-free (`aria-selected` toggling, `[data-tooltip]`,
native `<details>`).

---

## Appendix A — Token cheat-sheet

Every custom token, its value, and what to reach for it. Defined in
`tokens.css`. A **Dark** value appears only where the token changes in dark mode
(via `prefers-color-scheme` / `data-mode`); blank = identical in both. Reference
these in components — never raw literals or OpenProps props.

### Surfaces

| Token | Light | Dark | Use for |
| --- | --- | --- | --- |
| `--color-canvas` | `hsl(0 0% 100%)` | `hsl(200 28% 14.7%)` | card / region surface (on top) |
| `--color-canvas-hover` | `rgb(249 249 249)` | `hsl(200 28% 19%)` | faint fill feedback on tiles |
| `--color-page-tint` | = active tint | `hsl(202.5 42.1% 7.5%)` | the page behind (body bg) |

### Page tints (light only — collapse to one blue in dark)

| Token | Value | `data-tint` |
| --- | --- | --- |
| `--tint-warm` | `hsl(26 94% 98%)` | `warm` (default) |
| `--tint-green` | `hsl(135 29% 97%)` | `green` |
| `--tint-purple` | `hsl(277 100% 98.5%)` | `purple` |
| `--tint-blue` | `hsl(214 100% 98%)` | `blue` |
| `--tint-red` | `hsl(0 82% 97.5%)` | `red` |
| `--tint-grey` | `hsl(0 0% 96.5%)` | `grey` |
| `--tint-warm-2` | `hsl(26 94% 98%)` | (dup of default) |

### Ink & overlays

| Token | Light | Dark | Use for |
| --- | --- | --- | --- |
| `--ink-rgb` | `43 55 62` | `222 227 230` | triplet behind ink + overlays |
| `--color-ink` | `rgb(var(--ink-rgb))` | `hsl(205.7 13.2% 89.6%)` | headings + body text |
| `--color-ink-muted` | ink @ 60% | near-white @ 60% | secondary text |
| `--overlay-hover` | ink @ 8% | (auto via ink-rgb) | button/row hover fill |
| `--overlay-active` | ink @ 25% | (auto) | menu-item / pressed |

### Accents & state

| Token | Value | Use for |
| --- | --- | --- |
| `--color-blue` | `rgb(35 119 210)` | interactive: links, primary, menus |
| `--color-green` | `rgb(25 135 77)` | selected / affirmative / "on" |
| `--color-rust` | `oklch(53% 0.15 40)` | structural section-label eyebrows |
| `--color-danger` | `rgb(204 51 51)` | destructive actions / errors |
| `--color-warning` | `rgb(204 133 20)` | caution |
| `--color-hairline` | `rgb(227 229 229)` | 1px separators (dark `hsl(200 18% 24%)`) |

(`info` = blue, `success` = green — no separate tokens.)

### Elevation

| Token | Value | Tier / use |
| --- | --- | --- |
| `--shadow-halo` | `0 0 30px rgb(0 0 0 / .08)` (dark `.35`) | tier 0 cards |
| `--shadow-dialog` | 5-step even ramp → `0 16px 32px` | tier 2 floating dialog |
| `--shadow-modal` | 1px ring + `-8px`/`-16px` ambients | tier 3 modal panel |
| `--color-scrim` | `rgb(0 0 0 / 0.5)` | tier 3 backdrop |
| `--modal-width` | `500px` | tier 3 panel width |

### Shape, motion, measure

| Token | Value | Use for |
| --- | --- | --- |
| `--radius` | `8px` | cards, buttons, fields, menus |
| `--radius-lg` | `16px` | floating dialog |
| `--radius-modal` | `12px` | modal panel |
| `--measure` | `78ch` | content column max width |
| `--card-pad` | `var(--size-5)` (24px) | card / dialog / modal padding |
| `--ease-spring` | `cubic-bezier(.25,1.25,.5,1)` | menu overshoot (300ms) |
| `--ease-standard` | `cubic-bezier(.2,0,0,1)` | general transitions |
| `--duration-1` / `-2` | `125ms` / `200ms` | quick / standard |
| `--tint-transition` | `250ms ease` | re-tint / mode swap |

### Fields

| Token | Value | Use for |
| --- | --- | --- |
| `--field-height` | `40px` | input / select height |
| `--color-field-border` | ink @ 15% | input/select border |
| `--color-field-border-strong` | ink @ 25% | tile / segment border |
| `--select-arrow` | data-URI double-chevron | select arrow (near-white in dark) |
| `--select-arrow-size` | `8.8px` | select arrow size |

### Interaction & layering

| Token | Value | Use for |
| --- | --- | --- |
| `--focus-ring` | `0 0 0 2px var(--color-blue)` | custom focus box-shadow |
| `--disabled-opacity` | `0.5` | disabled controls |
| `--z-sticky` / `-menu` / `-dialog` / `-modal` / `-toast` | `10` / `20` / `100` / `300` / `400` | the only z-index values |
