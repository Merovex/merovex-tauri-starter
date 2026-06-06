// The document file extension, derived at build time from tauri.conf.json's
// file association (injected by vite.config.js as __DOC_EXT__). The SINGLE place
// to change it is tauri.conf.json → bundle.fileAssociations[0].ext — Rust
// (build.rs) and this frontend both follow. See CLAUDE.md.
/* global __DOC_EXT__ */
export const DOC_EXT = __DOC_EXT__;
export const DOC_FILTERS = [{ name: 'Documents', extensions: [DOC_EXT] }];
