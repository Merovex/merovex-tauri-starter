#!/usr/bin/env node
/* Writes a 1024×1024 solid-color PNG (Basecamp blue) to app-icon.png — a
 * placeholder source for `pnpm tauri icon`. Replace with real art later.
 * Pure Node (zlib only); no dependencies. */
import { deflateSync } from 'node:zlib';
import { writeFileSync } from 'node:fs';

const SIZE = 1024;
const RGB = [35, 119, 210]; // --color-blue

function crc32(buf) {
  let c = ~0;
  for (let i = 0; i < buf.length; i++) {
    c ^= buf[i];
    for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1));
  }
  return (~c) >>> 0;
}
function chunk(type, data) {
  const t = Buffer.from(type, 'ascii');
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([t, data])));
  return Buffer.concat([len, t, data, crc]);
}

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(SIZE, 0);
ihdr.writeUInt32BE(SIZE, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 2; // color type: RGB

// raw scanlines: filter byte (0) + RGB per pixel
const row = Buffer.alloc(1 + SIZE * 3);
for (let x = 0; x < SIZE; x++) {
  row[1 + x * 3] = RGB[0];
  row[1 + x * 3 + 1] = RGB[1];
  row[1 + x * 3 + 2] = RGB[2];
}
const raw = Buffer.concat(Array.from({ length: SIZE }, () => row));

const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk('IHDR', ihdr),
  chunk('IDAT', deflateSync(raw)),
  chunk('IEND', Buffer.alloc(0)),
]);

writeFileSync(new URL('../app-icon.png', import.meta.url), png);
console.log('✓ wrote app-icon.png (1024×1024). Now: pnpm tauri icon app-icon.png');
