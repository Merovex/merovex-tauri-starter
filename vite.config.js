import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';

// Single source of truth for the document extension: tauri.conf.json's file
// association. Injected here so the frontend never hardcodes it (see lib/doc.js).
const tauriConf = JSON.parse(
  readFileSync(resolve(__dirname, 'src-tauri/tauri.conf.json'), 'utf8')
);
const DOC_EXT = tauriConf.bundle?.fileAssociations?.[0]?.ext?.[0] ?? 'appdoc';

// Frontend lives in src/, builds to dist/ (which tauri.conf.json points at).
export default defineConfig({
  plugins: [svelte()],

  define: {
    __DOC_EXT__: JSON.stringify(DOC_EXT),
  },

  root: 'src',
  publicDir: '../public',

  build: {
    outDir: '../dist',
    emptyOutDir: true,
    target: 'esnext', // Tauri ships a modern system webview.
    sourcemap: true,
  },

  server: {
    port: 5173,
    strictPort: true,
    watch: { ignored: ['**/src-tauri/**'] },
  },

  resolve: {
    alias: {
      $lib: resolve(__dirname, 'src/lib'),
      $stores: resolve(__dirname, 'src/lib/stores'),
    },
  },
});
