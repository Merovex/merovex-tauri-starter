#!/usr/bin/env node
/* Enforces the hard rule: NO CSS in .svelte files.
 *
 * Fails the build if any .svelte file under src/ contains a <style> block.
 * All styling must live in src/styles/*.css. This is wired into `pnpm build`
 * (and therefore `pnpm tauri build`) so a violation literally cannot ship.
 *
 * Note on inline styles: dynamic `style="--token: {value}"` for passing a CSS
 * custom property is the one tolerated exception (it sets a variable, not
 * styling). Static inline CSS (e.g. style="color: red") is forbidden too —
 * keep it out, even though this script only hard-checks <style> blocks. */

import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const SRC = fileURLToPath(new URL('../src', import.meta.url));
// Match a <style> element at the start of a line (optionally indented) — i.e. a
// real Svelte style block, not a "<style>" mention inside a JS comment/string.
const STYLE_BLOCK = /^\s*<style[\s>]/im;

const offenders = [];

function walk(dir) {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    if (statSync(p).isDirectory()) {
      walk(p);
    } else if (p.endsWith('.svelte') && STYLE_BLOCK.test(readFileSync(p, 'utf8'))) {
      offenders.push(p);
    }
  }
}

walk(SRC);

if (offenders.length > 0) {
  console.error('\n\x1b[31m✖ CSS is not allowed in .svelte files.\x1b[0m');
  console.error('  Move all styling to src/styles/*.css (BEM classes + tokens).');
  console.error('  Offending files:');
  for (const f of offenders) console.error('    - ' + f);
  console.error('');
  process.exit(1);
}

console.log('\x1b[32m✓\x1b[0m No <style> blocks in .svelte files.');
