import { unlink } from "node:fs/promises";

const rawWasmFile = await Bun.file('pkg/wasm_bg.wasm').arrayBuffer();
const origJsFile = await Bun.file('pkg/wasm.js').text();

const base64 = Buffer.from(rawWasmFile).toString('base64');

const patchedJsFile = origJsFile
    .replace(`const path = require('path').join(__dirname, 'wasm_bg.wasm');`, '')
    .replace(', fatal: true', '')
    .replace(`const bytes = require('fs').readFileSync(path);`, `
const { Buffer } = require('node:buffer');
const bytes = Buffer.from('${base64}', 'base64');`)

await Bun.write('pkg/wasm.js', patchedJsFile);

// Remove wasm file
await unlink('pkg/wasm_bg.wasm');

// Remove wasm from .files section of package.json
const pkgJson = await Bun.file('pkg/package.json').json();
pkgJson.name = '@swc/wasm-typescript';
pkgJson.files = pkgJson.files.filter(file => file !== 'wasm_bg.wasm');
await Bun.write('pkg/package.json', JSON.stringify(pkgJson, null, 2));
