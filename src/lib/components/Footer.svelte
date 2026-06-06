<script>
  // Bottom utility row. Left: settings cog + tint "drop". Center: quick nav.
  // Right: notifications bell (placeholder). The cog opens a Preferences modal
  // holding the Appearance control. No styling here — see styles/components.css.
  import Icon from '$lib/components/Icon.svelte';
  import { tint, mode, cycleTint, confirmDestructive } from '$stores/prefs.svelte.js';
  import { toast } from '$stores/toast.svelte.js';
  import { ui } from '$stores/ui.svelte.js';
  import { notifications } from '$stores/notifications.svelte.js';

  const NAV = ['My Tasks', 'My Events', 'Due Today', 'My Bookmarks', 'My Notes'];

  function onKeydown(e) {
    if (e.key === 'Escape') ui.settingsOpen = false;
  }
</script>

<svelte:window onkeydown={onKeydown} />

<footer class="footer">
  <div class="footer__left">
    <button
      class="button button--icon"
      data-tooltip="Settings"
      aria-label="Settings"
      onclick={() => (ui.settingsOpen = true)}
    >
      <Icon name="gear" />
    </button>
    <button
      class="button button--icon"
      data-tooltip={`Tint: ${tint.value}`}
      aria-label="Change background tint"
      onclick={cycleTint}
    >
      <Icon name="drop" />
    </button>
  </div>

  <nav class="footer-nav" aria-label="Quick navigation">
    {#each NAV as item}
      <button class="navlink" onclick={() => toast.show(item)}>{item}</button>
    {/each}
  </nav>

  <div class="footer__right">
    {#if ui.notifOpen}
      <button
        class="button button--icon"
        aria-expanded="true"
        aria-label="Close notifications"
        onclick={() => ui.closeNotif()}
      >
        <Icon name="x" />
      </button>
    {:else if notifications.unread > 0}
      <button
        class="notif-toggle"
        aria-expanded="false"
        aria-label="Notifications"
        onclick={() => ui.toggleNotif()}
      >
        <span class="notif-count">{notifications.unread}</span>
        New for you
      </button>
    {:else}
      <button
        class="notif-toggle notif-toggle--empty"
        aria-expanded="false"
        aria-label="Notifications"
        onclick={() => ui.toggleNotif()}
      >
        <Icon name="bell" class="icon--sm" /> Notifications
      </button>
    {/if}
  </div>
</footer>

{#if ui.settingsOpen}
  <div
    class="modal"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) ui.settingsOpen = false;
    }}
  >
    <div class="modal__panel stack" role="dialog" aria-modal="true" aria-label="Preferences">
      <div class="cluster cluster--between">
        <h2>Preferences</h2>
        <button class="button button--icon" aria-label="Close" onclick={() => (ui.settingsOpen = false)}>
          <Icon name="x" />
        </button>
      </div>

      <div class="stack stack--tight">
        <p class="eyebrow">Appearance</p>
        <div class="segmented" role="radiogroup" aria-label="Color mode">
          <button
            class="segmented__option"
            role="radio"
            aria-checked={mode.value === 'light'}
            onclick={() => (mode.value = 'light')}
          >
            <Icon name="sun" class="icon--lg" /> Light
          </button>
          <button
            class="segmented__option"
            role="radio"
            aria-checked={mode.value === 'dark'}
            onclick={() => (mode.value = 'dark')}
          >
            <Icon name="moon-solid" class="icon--lg" /> Dark
          </button>
          <button
            class="segmented__option"
            role="radio"
            aria-checked={mode.value === 'auto'}
            onclick={() => (mode.value = 'auto')}
          >
            <Icon name="desktop" class="icon--lg" /> Auto
          </button>
        </div>
      </div>

      <div class="stack stack--tight">
        <p class="eyebrow">General</p>
        <label class="switch">
          <input type="checkbox" class="switch__input" bind:checked={confirmDestructive.value} />
          <span class="switch__track"></span>
          Confirm before deleting
        </label>
      </div>
    </div>
  </div>
{/if}
