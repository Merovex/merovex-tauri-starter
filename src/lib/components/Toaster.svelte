<script>
  // Renders the transient toast queue. Mount once near the app root.
  // Styling is external (.toaster/.toast); exit uses a JS transition (the
  // element unmounts, so CSS alone can't animate removal).
  import { fade } from 'svelte/transition';
  import { toast } from '$stores/toast.svelte.js';
</script>

<div class="toaster" aria-live="polite" aria-atomic="false">
  {#each toast.items as t (t.id)}
    <div
      class={`toast ${t.type !== 'default' ? `toast--${t.type}` : ''}`}
      role="status"
      transition:fade={{ duration: 150 }}
    >
      {t.message}
    </div>
  {/each}
</div>
