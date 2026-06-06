// Thin, typed-ish wrappers over Tauri's invoke(). Keep ALL invoke() calls here
// so the rest of the UI never touches command-name strings directly. Each
// function maps 1:1 to a #[tauri::command] in src-tauri/src/commands.rs.

import { invoke } from '@tauri-apps/api/core';

// --- App ------------------------------------------------------------------

export const appInfo = () => invoke('app_info');

// --- Document lifecycle ---------------------------------------------------

export const openDocument = (docPath) => invoke('open_document', { docPath });
export const listRecent = () => invoke('list_recent');

// --- Items (placeholder domain) -------------------------------------------

export const optimizeDocument = (docPath) => invoke('optimize_document', { docPath });

export const listItems = (docPath) => invoke('list_items', { docPath });
export const createItem = (docPath, title) => invoke('create_item', { docPath, title });
export const updateItem = (docPath, id, title, body) =>
  invoke('update_item', { docPath, id, title, body });
export const deleteItem = (docPath, id) => invoke('delete_item', { docPath, id });
