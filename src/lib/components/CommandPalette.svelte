<script>
  // ⌘K command palette. Receives a list of { label, icon, run } actions and
  // runs the chosen one. Open state lives in the ui store. Styling is external.
  import Icon from '$lib/components/Icon.svelte';
  import { ui } from '$stores/ui.svelte.js';

  let { actions = [] } = $props();
  let query = $state('');

  const filtered = $derived(
    actions.filter((a) => a.label.toLowerCase().includes(query.trim().toLowerCase()))
  );

  function close() {
    ui.paletteOpen = false;
    query = '';
  }
  function run(action) {
    close();
    action.run();
  }
  function onKeydown(e) {
    if (e.key === 'Escape') close();
    else if (e.key === 'Enter' && filtered[0]) run(filtered[0]);
  }
</script>

{#if ui.paletteOpen}
  <div
    class="palette-backdrop"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) close();
    }}
  >
    <div class="palette" role="dialog" aria-modal="true" aria-label="Command palette">
      <!-- svelte-ignore a11y_autofocus -->
      <input
        class="palette__input"
        placeholder="Type a command…"
        bind:value={query}
        onkeydown={onKeydown}
        autofocus
      />
      <ul class="palette__list plain-list">
        {#each filtered as action (action.label)}
          <li>
            <button class="palette__item" onclick={() => run(action)}>
              <Icon name={action.icon} class="icon--sm" />
              {action.label}
            </button>
          </li>
        {/each}
        {#if filtered.length === 0}
          <li class="palette__empty muted">No commands</li>
        {/if}
      </ul>
    </div>
  </div>
{/if}
