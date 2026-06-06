// Notifications model. Sample data for the starter — replace with a real feed.
// `unread` drives the footer toggle (count + "New for you" vs "Notifications").

class Notifications {
  items = $state([
    {
      id: 3,
      title: "Due today: Sign up for TSA's pre-check via US Customs & Border Patrol's Global Entry Trusted Trav...",
      meta: '16:59 · Basecamp · 4.0 - Windstream Way',
      read: false,
    },
    { id: 2, title: 'Due soon: Avalon Brakes', meta: '16:34 · Basecamp · Pink Flamingo Universe', read: true },
    { id: 4, title: 'Today: Mosquito Spray', meta: '16:34 · Basecamp · 4.0 - Windstream Way', read: true },
    {
      id: 5,
      title: 'Due today: Blog — Building a World on a Companion Planet',
      meta: '12:28 · Basecamp · Merovex Marketing',
      read: true,
    },
  ]);

  // Reminders the user set. `active: true` = the reminder has come due (shown at
  // the top under "Reminders"); `active: false` = still scheduled (in the
  // "Scheduled" list + counted in the link). Seeded for the test.
  reminders = $state([
    {
      id: 10,
      title: '📣 Community Highlights | Cycle 1 — Hi Community! You’re all busy with work and life so I’ve pulled together a…',
      meta: 'Mar 18, 2025 · Ashley B. · Community HQ',
      active: true,
    },
    {
      id: 11,
      title: '💡 Family Basecamp — I have a daughter who has Down syndrome that just turned 18. We did…',
      meta: 'Jan 24, 2025 · Jethro Jones · Community',
      active: true,
    },
    {
      id: 1,
      title: "What did you finish today? What is Tomorrow's Frog? (And record MFP)",
      meta: '16:36 · Basecamp · Team: Wilson Family',
      when: 'Jun 7',
      active: false,
    },
  ]);

  get due() {
    return this.reminders.filter((n) => n.active);
  }

  get scheduled() {
    return this.reminders.filter((n) => !n.active);
  }

  get scheduledCount() {
    return this.scheduled.length;
  }

  get unread() {
    return this.items.filter((n) => !n.read).length;
  }

  markAllRead() {
    this.items = this.items.map((n) => ({ ...n, read: true }));
  }

  setRead(id, value) {
    this.items = this.items.map((n) => (n.id === id ? { ...n, read: value } : n));
  }

  // Schedule a reminder — from the feed, or re-schedule one that came due.
  remind(id, when) {
    let item = this.items.find((n) => n.id === id);
    if (item) {
      this.items = this.items.filter((n) => n.id !== id);
    } else {
      item = this.reminders.find((n) => n.id === id);
      this.reminders = this.reminders.filter((n) => n.id !== id);
    }
    if (!item) return;
    this.reminders = [...this.reminders, { ...item, when, active: false }];
  }

  // Show a due reminder in the feed now (as unread "new for you").
  showNow(id) {
    const item = this.reminders.find((n) => n.id === id);
    if (!item) return;
    this.reminders = this.reminders.filter((n) => n.id !== id);
    const { when, active, ...rest } = item;
    this.items = [{ ...rest, read: false }, ...this.items];
  }

  remove(id) {
    this.items = this.items.filter((n) => n.id !== id);
    this.reminders = this.reminders.filter((n) => n.id !== id);
  }
}

export const notifications = new Notifications();

// Singleton store: force a full reload on edit (avoids HMR instance desync).
if (import.meta.hot) import.meta.hot.decline();
