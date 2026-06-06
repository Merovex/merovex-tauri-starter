<script>
  // By design, this file carries no styling — no style element, no inline CSS.
  // All styling lives in src/styles/*.css. See CLAUDE.md.
  import { open, save, confirm } from '@tauri-apps/plugin-dialog';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { DOC_EXT, DOC_FILTERS } from '$lib/doc.js';
  import { appInfo } from '$lib/api.js';
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import { documentStore } from '$stores/document.svelte.js';
  import { toast } from '$stores/toast.svelte.js';
  import { tint, mode, confirmDestructive } from '$stores/prefs.svelte.js';
  import { ui } from '$stores/ui.svelte.js';
  import Icon from '$lib/components/Icon.svelte';
  import Toaster from '$lib/components/Toaster.svelte';
  import Footer from '$lib/components/Footer.svelte';
  import Notifications from '$lib/components/Notifications.svelte';
  import CommandPalette from '$lib/components/CommandPalette.svelte';

  let newTitle = $state('');
  let about = $state(null); // app info for the About modal

  $effect(() => {
    documentStore.loadRecent();
  });

  // Re-tint the whole app: one attribute on <html> re-colors the page.
  $effect(() => {
    document.documentElement.dataset.tint = tint.value;
  });

  // Mode: 'auto' defers to the system; otherwise force it.
  $effect(() => {
    const el = document.documentElement;
    if (mode.value === 'auto') el.removeAttribute('data-mode');
    else el.dataset.mode = mode.value;
  });

  // Native menu + "open with" events from the backend.
  $effect(() => {
    const unlistens = [];
    let active = true;
    (async () => {
      unlistens.push(await listen('menu:new', () => createNew()));
      unlistens.push(await listen('menu:open', () => openExisting()));
      unlistens.push(await listen('menu:preferences', () => (ui.settingsOpen = true)));
      unlistens.push(await listen('menu:about', () => (ui.aboutOpen = true)));
      unlistens.push(await listen('menu:close-doc', () => documentStore.close()));
      unlistens.push(await listen('menu:check-update', () => checkForUpdates()));
      unlistens.push(await listen('open-document', (e) => documentStore.open(e.payload)));
      if (!active) unlistens.forEach((u) => u());
    })();
    return () => {
      active = false;
      unlistens.forEach((u) => u());
    };
  });

  // Load app info once (used by the title bar and the About modal).
  $effect(() => {
    if (!about) appInfo().then((i) => (about = i));
  });

  // Window title: "App Name: Document" when a document is open, else "App Name".
  $effect(() => {
    const base = about?.name ?? 'Tauri Starter';
    let title = base;
    if (documentStore.path) {
      const file = documentStore.path.split(/[/\\]/).pop() ?? '';
      const name = file.replace(new RegExp(`\\.${DOC_EXT}$`), '');
      title = `${base}: ${name}`;
    }
    getCurrentWindow().setTitle(title);
  });

  // Drag a document onto the window to open it.
  $effect(() => {
    let un;
    getCurrentWebview()
      .onDragDropEvent((e) => {
        if (e.payload.type === 'drop') {
          const f = e.payload.paths.find((p) => p.toLowerCase().endsWith(`.${DOC_EXT}`));
          if (f) documentStore.open(f);
        }
      })
      .then((u) => (un = u));
    return () => un?.();
  });

  // Surface uncaught errors / rejected commands instead of failing silently.
  $effect(() => {
    const onErr = (e) => toast.danger(e.message ?? 'Unexpected error');
    const onRej = (e) => toast.danger(`Action failed: ${e.reason?.message ?? e.reason ?? ''}`);
    window.addEventListener('error', onErr);
    window.addEventListener('unhandledrejection', onRej);
    return () => {
      window.removeEventListener('error', onErr);
      window.removeEventListener('unhandledrejection', onRej);
    };
  });

  // Command palette (⌘K) actions.
  const commands = [
    { label: 'New document', icon: 'file-draft', run: () => createNew() },
    { label: 'Open document', icon: 'folder', run: () => openExisting() },
    { label: 'Close document', icon: 'x', run: () => documentStore.close() },
    { label: 'Preferences', icon: 'gear', run: () => (ui.settingsOpen = true) },
    { label: 'Notifications', icon: 'bell', run: () => ui.toggleNotif() },
    { label: 'About', icon: 'question-circle', run: () => (ui.aboutOpen = true) },
    { label: 'Check for updates', icon: 'download', run: () => checkForUpdates() },
  ];

  function onGlobalKey(e) {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      ui.paletteOpen = !ui.paletteOpen;
    } else if (e.key === 'Escape') {
      ui.aboutOpen = false;
      ui.paletteOpen = false;
    }
  }

  async function openExisting() {
    const path = await open({
      multiple: false,
      filters: DOC_FILTERS,
    });
    if (path) await documentStore.open(path);
  }

  async function createNew() {
    const path = await save({
      filters: DOC_FILTERS,
      defaultPath: `Untitled.${DOC_EXT}`,
    });
    if (path) await documentStore.open(path); // creates if missing
  }

  async function addItem() {
    const title = newTitle.trim();
    if (!title) return;
    await documentStore.addItem(title);
    newTitle = '';
    toast.success(`Added “${title}”`);
  }

  async function removeItem(item) {
    if (confirmDestructive.value) {
      const ok = await confirm(`Delete “${item.title}”?`, { title: 'Delete', kind: 'warning' });
      if (!ok) return;
    }
    await documentStore.removeItem(item.id);
    toast.show('Item deleted');
  }

  async function checkForUpdates() {
    try {
      const update = await check();
      if (!update) {
        toast.show('You’re up to date');
        return;
      }
      const ok = await confirm(`Update to ${update.version}? It will install and restart.`, {
        title: 'Update available',
        kind: 'info',
      });
      if (!ok) return;
      toast.show('Downloading update…');
      await update.downloadAndInstall();
      await relaunch();
    } catch (e) {
      toast.danger(`Update check failed: ${e?.message ?? e}`);
    }
  }
</script>

<svelte:window onkeydown={onGlobalKey} />

<svelte:boundary>
<div class="app-shell" class:is-pushed={ui.notifOpen}>
<main class="center stack stack--loose">
  {#if documentStore.isOpen}
    <div class="stack">
      <form class="cluster" onsubmit={(e) => { e.preventDefault(); addItem(); }}>
        <input class="field grow" placeholder="New item title…" bind:value={newTitle} />
        <button class="button button--primary" type="submit">
          <Icon name="plus-solid" /> Add
        </button>
      </form>

      <p class="eyebrow">Items</p>
      {#if documentStore.items.length === 0}
        <p class="muted">No items yet. Add one above.</p>
      {:else}
        <ul class="grid plain-list">
          {#each documentStore.items as item (item.id)}
            <li class="card stack stack--tight">
              <h2 class="card__title">{item.title}</h2>
              <div>
                <button
                  class="button button--danger"
                  data-tooltip="Move to trash"
                  onclick={() => removeItem(item)}
                >
                  <Icon name="trash" /> Delete
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {:else}
    <div class="stack">
      <p class="muted">No document open. Create or open a <code>.{DOC_EXT}</code> file to begin.</p>

      <div class="cluster">
        <button class="button button--primary" onclick={createNew}>
          <Icon name="file-draft" /> New document
        </button>
        <button class="button" onclick={openExisting}>
          <Icon name="folder" /> Open…
        </button>
      </div>

      {#if documentStore.recent.length > 0}
        <p class="eyebrow">Recent</p>
        <ul class="grid plain-list">
          {#each documentStore.recent as r (r.id)}
            <li class="card stack stack--tight">
              <h2 class="card__title">{r.title}</h2>
              <p class="card__meta">{r.path}</p>
              <div>
                <button class="button" onclick={() => documentStore.open(r.path)}>
                  <Icon name="arrow-right" /> Open
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</main>
</div>

{#snippet failed(error, reset)}
  <div class="center">
    <div class="banner banner--danger">
      <Icon name="alert-circle" class="banner__icon" />
      <div class="banner__body">
        <p>Something went wrong.</p>
        <button class="button" onclick={reset}>Try again</button>
      </div>
    </div>
  </div>
{/snippet}
</svelte:boundary>

<Notifications />
<Footer />
<Toaster />
<CommandPalette actions={commands} />

{#if ui.aboutOpen}
  <div
    class="modal"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) ui.aboutOpen = false;
    }}
  >
    <div class="modal__panel stack" role="dialog" aria-modal="true" aria-label="About">
      <div class="cluster cluster--between">
        <h2>{about?.name ?? 'Tauri Starter'}</h2>
        <button class="button button--icon" aria-label="Close" onclick={() => (ui.aboutOpen = false)}>
          <Icon name="x" />
        </button>
      </div>
      <div class="stack stack--tight">
        <p class="muted">Version {about?.version ?? '—'}</p>
        <p class="muted">Built {about?.build_date ?? '—'}</p>
        <p class="muted">Document type: .{about?.doc_ext ?? DOC_EXT}</p>
      </div>
    </div>
  </div>
{/if}
