// Transient notifications. Components call toast.success(...) / .danger(...) /
// .show(...); <Toaster> renders the queue. Styling is the .toast/.toaster CSS.

class ToastStore {
  items = $state([]);
  #seq = 0;

  show(message, type = 'default', ms = 3000) {
    const id = ++this.#seq;
    this.items = [...this.items, { id, message, type }];
    if (ms) setTimeout(() => this.dismiss(id), ms);
    return id;
  }

  success(message, ms) {
    return this.show(message, 'success', ms);
  }
  danger(message, ms) {
    return this.show(message, 'danger', ms);
  }

  dismiss(id) {
    this.items = this.items.filter((t) => t.id !== id);
  }
}

export const toast = new ToastStore();

// Singleton store: force a full reload on edit (avoids HMR instance desync).
if (import.meta.hot) import.meta.hot.decline();
