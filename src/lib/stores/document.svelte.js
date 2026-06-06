// Reactive document state using Svelte 5 runes. Components import this singleton
// and read its fields reactively; mutations flow through its methods, which call
// the backend and keep local state in sync.
//
// This is the "Svelte store as reactive state" channel. For one-off signals
// between unrelated components, prefer a DOM CustomEvent instead of widening
// this store.

import * as api from '$lib/api.js';

class DocumentStore {
  /** Absolute path to the open .appdoc file, or null. */
  path = $state(null);
  /** Items in the open document. */
  items = $state([]);
  /** Recently opened documents (from app.db). */
  recent = $state([]);

  get isOpen() {
    return this.path !== null;
  }

  async open(path) {
    await api.openDocument(path);
    this.path = path;
    await this.refresh();
  }

  // Close the open document. Optimizes (checkpoint + VACUUM) on the way out,
  // then returns to the no-document state. Best-effort optimize.
  async close() {
    if (this.path) {
      try {
        await api.optimizeDocument(this.path);
      } catch {
        // ignore — closing shouldn't fail because optimize did
      }
    }
    this.path = null;
    this.items = [];
  }

  async refresh() {
    if (!this.path) return;
    this.items = await api.listItems(this.path);
  }

  async loadRecent() {
    this.recent = await api.listRecent();
  }

  async addItem(title) {
    if (!this.path) return;
    const item = await api.createItem(this.path, title);
    this.items = [...this.items, item];
    return item;
  }

  async saveItem(id, title, body) {
    if (!this.path) return;
    await api.updateItem(this.path, id, title, body);
    this.items = this.items.map((it) =>
      it.id === id ? { ...it, title, body } : it
    );
  }

  async removeItem(id) {
    if (!this.path) return;
    await api.deleteItem(this.path, id);
    this.items = this.items.filter((it) => it.id !== id);
  }
}

export const documentStore = new DocumentStore();

// Singleton store: force a full reload on edit (avoids HMR instance desync).
if (import.meta.hot) import.meta.hot.decline();
