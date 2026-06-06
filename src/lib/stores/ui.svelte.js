// Cross-component UI state (panels, etc.).

class UI {
  notifOpen = $state(false);
  settingsOpen = $state(false); // Preferences modal
  aboutOpen = $state(false); // About modal
  paletteOpen = $state(false); // command palette (⌘K)

  toggleNotif() {
    this.notifOpen = !this.notifOpen;
  }
  closeNotif() {
    this.notifOpen = false;
  }
}

export const ui = new UI();

// Singleton store: force a full reload on edit so HMR never leaves components
// bound to a stale instance.
if (import.meta.hot) import.meta.hot.decline();
