<script>
  // Right-side push panel (fixed width). Two views: the main feed and a
  // "Scheduled" list (reminders not yet due). Due reminders sit at the top under
  // "Reminders". Their menu adds Show now / Cancel reminder and drops "Now".
  import Icon from '$lib/components/Icon.svelte';
  import { ui } from '$stores/ui.svelte.js';
  import { notifications } from '$stores/notifications.svelte.js';
  import { toast } from '$stores/toast.svelte.js';

  let filter = $state('');
  let view = $state('main'); // 'main' | 'scheduled'
  let openMenu = $state(null); // { id, type: 'more' | 'remind' | 'reminder' } | null
  let picking = $state(null); // id of the row whose "Pick a date" panel is open
  let pickDate = $state(''); // YYYY-MM-DD

  const q = $derived(filter.trim().toLowerCase());
  const match = (n) => n.title.toLowerCase().includes(q);
  const newItems = $derived(notifications.items.filter((n) => !n.read && match(n)));
  const prevItems = $derived(notifications.items.filter((n) => n.read && match(n)));

  // Remind targets, with real dates computed once on mount.
  function targetDate(key) {
    const d = new Date();
    d.setHours(8, 0, 0, 0);
    const day = d.getDay();
    if (key === 'tomorrow') d.setDate(d.getDate() + 1);
    else if (key === 'weekend') d.setDate(d.getDate() + (((6 - day) % 7) || 7));
    else if (key === 'week') d.setDate(d.getDate() + (((1 - day) + 7) % 7 || 7));
    else if (key === 'pick') d.setDate(d.getDate() + 1);
    return d;
  }
  const TIME = '8:00am';
  const wk = (d) => d.toLocaleDateString(undefined, { weekday: 'short' }) + ', ' + TIME;
  const shortDate = (d) => d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
  const REMIND = [{ key: 'now', label: 'Now' }, { key: 'tomorrow', label: 'Tomorrow' }, { key: 'weekend', label: 'Next weekend' }, { key: 'week', label: 'Next week' }, { key: 'pick', label: 'Pick a date' }].map(
    (o) => (o.key === 'now' ? o : { ...o, when: o.key === 'pick' ? TIME : wk(targetDate(o.key)), short: shortDate(targetDate(o.key)) })
  );
  const REMIND_LATER = REMIND.filter((o) => o.key !== 'now');

  function isoPlusDays(days) {
    const d = new Date();
    d.setDate(d.getDate() + days);
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }
  function toggleMenu(id, type) {
    picking = null;
    openMenu = openMenu && openMenu.id === id && openMenu.type === type ? null : { id, type };
  }
  function close() {
    openMenu = null;
    picking = null;
  }
  function remind(n, option) {
    if (option.key === 'now') {
      close();
      return;
    }
    if (option.key === 'pick') {
      picking = n.id; // expand the inline date panel; don't close
      if (!pickDate) pickDate = isoPlusDays(1);
      return;
    }
    close();
    notifications.remind(n.id, option.short);
    toast.show(`Reminder set • ${option.label.toLowerCase()}`);
  }
  function schedulePicked(n) {
    if (!pickDate) return;
    const [y, m, d] = pickDate.split('-').map(Number);
    const when = shortDate(new Date(y, m - 1, d));
    close();
    notifications.remind(n.id, when);
    toast.show(`Reminder set • ${when}`);
  }
  function showNow(n) {
    close();
    notifications.showNow(n.id);
    toast.show('Shown in your feed');
  }
  function cancelReminder(n) {
    close();
    notifications.remove(n.id);
    toast.show('Reminder canceled');
  }
  function onWindowClick(e) {
    if (openMenu && !e.target.closest('.menu, .notif-row__actions, .notif-row__remind')) close();
  }
  function onKeydown(e) {
    if (e.key === 'Escape') close();
  }
</script>

<svelte:window onclick={onWindowClick} onkeydown={onKeydown} />

{#snippet remindOptions(n, options)}
  {#each options as option (option.key)}
    <button class="menu__item" onclick={() => remind(n, option)}>
      <Icon name="clock" class="icon--sm" />
      {option.label}
      {#if option.when}<span class="menu__meta">{option.when}</span>{/if}
    </button>
  {/each}
  {#if picking === n.id}
    <div class="menu__pick">
      <input type="date" class="field" bind:value={pickDate} aria-label="Pick a date" />
      <button class="menu__schedule" onclick={() => schedulePicked(n)}>Schedule reminder</button>
    </div>
  {/if}
{/snippet}

{#snippet remindMenu(n)}
  <div class="menu notif-menu notif-menu--wide">
    <p class="menu__header">Remind me later…</p>
    {@render remindOptions(n, REMIND)}
  </div>
{/snippet}

{#snippet reminderMenu(n)}
  <div class="menu notif-menu notif-menu--wide">
    <div class="menu__top">
      <button class="menu__act" onclick={() => showNow(n)}>
        <Icon name="bell" class="icon--sm" /> Show now
      </button>
      <button class="menu__act" onclick={() => cancelReminder(n)}>Cancel reminder</button>
    </div>
    <p class="menu__header">Or remind me later…</p>
    {@render remindOptions(n, REMIND_LATER)}
  </div>
{/snippet}

{#snippet row(n)}
  <div class="notif-row">
    <Icon name="bell-solid" class="notif-row__icon icon--lg" />
    <div class="notif-row__body">
      <h4 class="notif-row__title">{n.title}</h4>
      <div class="card__meta">{n.meta}</div>
    </div>

    <div class="notif-row__end">
      {#if !n.read}<span class="notif-count notif-row__count">1</span>{/if}
      <div class="notif-row__actions">
        <button class="notif-row__act" aria-label="Remind me later" onclick={() => toggleMenu(n.id, 'remind')}>
          <Icon name="clock" class="icon--sm" />
        </button>
        <button class="notif-row__act" aria-label="More actions" onclick={() => toggleMenu(n.id, 'more')}>
          <Icon name="overflow" class="icon--sm" />
        </button>
      </div>
    </div>

    {#if openMenu?.id === n.id && openMenu.type === 'more'}
      <div class="menu notif-menu">
        <button
          class="menu__item"
          onclick={() => {
            notifications.setRead(n.id, !n.read);
            close();
          }}
        >
          {n.read ? 'Mark as unread' : 'Mark as read'}
        </button>
      </div>
    {/if}

    {#if openMenu?.id === n.id && openMenu.type === 'remind'}{@render remindMenu(n)}{/if}
  </div>
{/snippet}

{#snippet reminderRow(n)}
  <div class="notif-row">
    <Icon name="bell-solid" class="notif-row__icon icon--lg" />
    <div class="notif-row__body">
      <h4 class="notif-row__title">{n.title}</h4>
      <div class="card__meta">{n.meta}</div>
    </div>
    <button class="notif-row__remind" aria-label="Reminder options" onclick={() => toggleMenu(n.id, 'reminder')}>
      <Icon name="clock" />
    </button>
    {#if openMenu?.id === n.id && openMenu.type === 'reminder'}{@render reminderMenu(n)}{/if}
  </div>
{/snippet}

<aside class="notif" class:is-open={ui.notifOpen} aria-label="Notifications" inert={!ui.notifOpen}>
  <div class="notif__inner">
    {#if view === 'scheduled'}
      <div class="notif__head">
        <button class="button button--icon" aria-label="Back" onclick={() => (view = 'main')}>
          <Icon name="arrow-left" />
        </button>
        <h2 class="notif__title">Scheduled</h2>
      </div>
      <div class="notif__scroll stack">
        {#if notifications.scheduled.length}
          {#each notifications.scheduled as n (n.id)}
            <div class="notif-row">
              <Icon name="bell-solid" class="notif-row__icon icon--lg" />
              <div class="notif-row__body">
                <h4 class="notif-row__title">{n.title}</h4>
                <div class="card__meta">{n.meta}</div>
              </div>
              <span class="remind-pill"><Icon name="clock" class="icon--sm" /> {n.when}</span>
            </div>
          {/each}
        {:else}
          <p class="muted">Nothing scheduled.</p>
        {/if}
      </div>
    {:else}
      <div class="notif__scroll stack">
        {#if notifications.due.length}
          <p class="notif__heading notif__heading--reminders">Reminders</p>
          {#each notifications.due as n (n.id)}
            {@render reminderRow(n)}
          {/each}
        {/if}

        <div class="cluster cluster--between">
          <p class="eyebrow">New for you</p>
          <button class="navlink" onclick={() => notifications.markAllRead()}>Mark all read</button>
        </div>

        {#if notifications.scheduledCount > 0}
          <button class="notif__scheduled-link" onclick={() => (view = 'scheduled')}>
            {notifications.scheduledCount} reminder{notifications.scheduledCount === 1 ? '' : 's'} scheduled…
          </button>
        {/if}

        {#if newItems.length}
          {#each newItems as n (n.id)}
            {@render row(n)}
          {/each}
        {:else}
          <div class="notif-empty">
            <Icon name="sparkles" class="icon--lg notif-empty__spark" />
            <p>Nothing new for you.</p>
          </div>
        {/if}

        {#if prevItems.length}
          <p class="notif__heading">Previous notifications</p>
          {#each prevItems as n (n.id)}
            {@render row(n)}
          {/each}
        {/if}
      </div>

      <div class="notif__filter">
        <input class="field" placeholder="Filter…" bind:value={filter} />
      </div>
    {/if}
  </div>
</aside>
