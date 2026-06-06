<script>
  // Round avatar with an initials fallback. Styling is external (.avatar).
  let { src = null, name = '', class: klass = '' } = $props();
  const initials = name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((s) => s[0].toUpperCase())
    .join('');
</script>

<span class={`avatar ${klass}`} role="img" aria-label={name || 'avatar'}>
  {#if src}
    <img {src} alt={name} />
  {:else}
    {initials}
  {/if}
</span>
