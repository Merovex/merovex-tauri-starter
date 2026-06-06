# Icon sprite inventory

Source: `public/sprites.svg` — one inline SVG sprite, **267 symbols**.

- Every glyph is `viewBox="0 0 24 24"` and `fill="currentColor"` (no strokes),
  so icons inherit the surrounding text color and recolor across themes/modes
  with no extra assets.
- **93** glyphs are `-solid` weights paired with an outline base.
- **15** are third-party brand logos (`logo-*`).

## Usage

Use the `Icon` component — never hand-roll an `<svg>`:

```svelte
<Icon name="bell" />                  <!-- outline, inherits color, 20px -->
<Icon name="bell-solid" />            <!-- solid weight -->
<Icon name="check" class="icon--sm" />   <!-- 14px (menu/inline default) -->
<Icon name="trash" label="Delete" />     <!-- labelled → role="img" -->
```

It resolves to `<use href="/sprites.svg#NAME">`. Sizes are class-driven:
`.icon` (20px) · `.icon--sm` (14px) · `.icon--lg` (24px native).

**Legend:** `+` after a name means a paired `-solid` variant also exists
(e.g. `bell` + → `bell` and `bell-solid`).

---

## Navigation, arrows & chevrons

`home` · `arrow-up` · `arrow-down` · `arrow-left` · `arrow-right` ·
`arrow-up-left` · `arrow-up-right` · `arrow-down-left` · `arrow-down-right` ·
`arrow-up-and-down` · `arrow-up-circle` + · `arrow-down-circle` + ·
`arrow-left-circle` + · `arrow-right-circle` + · `chevron-up` · `chevron-down` ·
`chevron-left` · `chevron-right` · `chevron-up-circle` + ·
`chevron-down-circle` + · `chevron-left-circle` + · `chevron-right-circle` + ·
`chevrons-y` · `triangle-up` + · `triangle-down` + · `triangle-left` + ·
`triangle-right` + · `corner-up-left` · `corner-up-right` · `corner-down-left` ·
`corner-down-right` · `collapse` · `expand` · `minimize` · `external-link` + ·
`forward` + · `reply` + · `merge` · `switch` + · `cycle` · `recurrence`

## Status, feedback & playback

`alert-circle` + · `check` + · `check-circle` + · `check-square` + · `circle` ·
`x` + · `x-circle` + · `x-square` + · `question` · `question-circle` + ·
`play` · `pause` · `pause-circle` + · `stop` · `star` + · `sparkles` +

## Files, content & docs

`file` + · `file-draft` + · `file-text` + · `file-drop` + · `folder` + ·
`copy` + · `clipboard` + · `journal` + · `template` · `card` + · `card-check` + ·
`card-move` + · `card-table` + · `attachment` · `download` + · `upload` + ·
`drive` + · `grid` + · `list` · `list-add` · `list-checks` · `rename` ·
`text-options` · `archive` + · `archive-arrow` + · `trash` + · `trash-clock` + ·
`bookmark` + · `pin` +

## Communication & messaging

`bell` + · `bell-off` + · `message` + · `message-off` + · `message-text` + ·
`messages` + · `bubble` · `bubble-outline` · `bubble-pop` · `bubble-up` ·
`email` + · `email-sent` + · `mention` · `answer` · `megaphone` + · `inbox` + ·
`microphone` + · `microphone-disabled` + · `chatbot` + · `boost` + · `reply` + ·
`forward` +

## People, orgs & places

`person` + · `person-add` + · `people` + · `group` + · `crown` + · `face` + ·
`face-add` + · `building` + · `briefcase` + · `nonprofit` · `education` ·
`tent` · `world` · `globe` · `public-link` · `link`

## Time, charts & data

`calendar` + · `clock` + · `clock-back` · `stopwatch` + · `recurrence` ·
`chart` · `pie-chart` + · `gauge` · `gauge-circle` + · `hill-chart` ·
`hill-chart-square` + · `memory` + · `memory-remove` + · `lineup` +

## Tools, settings & input

`plus` + · `plus-circle` + · `pencil` + · `gear` + · `sliders` + · `tools` + ·
`filter` · `search` · `zoom` · `drag` · `drag-handle` · `cursor-drag` · `key` ·
`lock` + · `password` + · `eye` + · `dollar`

## System, media & misc

`desktop` · `sun` + · `moon` + · `drop` · `overflow` · `video` + · `webhooks`

## Brand logos (`logo-*`)

`logo-adobe-creative-cloud` · `logo-airtable` · `logo-apple` · `logo-basecamp` ·
`logo-box` · `logo-dropbox` · `logo-figma` · `logo-google-drive` ·
`logo-icloud` · `logo-invision` · `logo-notion` · `logo-one-drive` ·
`logo-sketch` · `logo-twitter` · `logo-zoho`

---

## Basecamp-specific glyphs worth knowing

`boost` (reactions) · `answer` · `bubble-pop` · `hill-chart` /
`hill-chart-square` (project progress) · `lineup` (schedule view) · `drop`
(the tint switcher) · `megaphone` · `mention` · `campfire`-style `messages`.

> The categories above are for browsing convenience; the only source of truth is
> the sprite itself. To regenerate the raw list:
> `grep -oE 'id="[^"]*"' public/sprites.svg` (filter to `<symbol>` ids).
