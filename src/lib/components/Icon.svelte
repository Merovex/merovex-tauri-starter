<script>
  // Renders one glyph from the shared sprite at public/sprites.svg.
  // All icons are 24×24, fill: currentColor — they inherit the surrounding
  // text color, so they recolor across themes/modes with no extra assets.
  // Styling lives in CSS (.icon); only class names cross this boundary.
  let { name, label = null, class: klass = '' } = $props();
</script>

<svg
  class={`icon ${klass}`}
  role={label ? 'img' : null}
  aria-label={label}
  aria-hidden={label ? null : 'true'}
>
  <use href={`/sprites.svg#${name}`} />
</svg>
