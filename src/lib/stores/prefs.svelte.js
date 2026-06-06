// App-wide UI preferences, persisted to localStorage. Shared by the app root
// (applies them to <html>) and the Footer/Preferences modal (edits them).

import { persisted } from './persisted.svelte.js';

export const tint = persisted('tint', 'warm'); // page-tint (the "drop")
export const mode = persisted('mode', 'auto'); // auto | light | dark
export const confirmDestructive = persisted('confirmDestructive', true); // ask before deletes

export const TINTS = ['warm', 'green', 'purple', 'blue', 'red', 'grey'];

export function cycleTint() {
  tint.value = TINTS[(TINTS.indexOf(tint.value) + 1) % TINTS.length];
}

// Singleton store: force a full reload on edit (avoids HMR instance desync).
if (import.meta.hot) import.meta.hot.decline();
