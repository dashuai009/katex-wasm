#!/usr/bin/env node
const path = require('path');
const { pathToFileURL } = require('url');
const { performance } = require('perf_hooks');

const ROOT = path.resolve(__dirname, '..');
const wasmPkgPath = pathToFileURL(path.join(ROOT, 'pkg', 'katex_wasm.js')).href;

function randomBytes(len) {
  const arr = new Uint8Array(len);
  for (let i = 0; i < len; i++) arr[i] = i & 0xff;
  return arr;
}

async function main() {
  const wasm = await import(wasmPkgPath);
  const bytes = Number(process.env.BYTES || 1 << 20);
  const rounds = Number(process.env.ROUNDS || 400);
  const input = randomBytes(bytes);
  const ptr = wasm.bench_alloc(bytes);
  const wasmMem = new Uint8Array(wasm.memory.buffer);

  let writeMs = 0;
  let readMs = 0;
  let computeMs = 0;
  let checksum = 0n;

  for (let i = 0; i < rounds; i++) {
    let t0 = performance.now();
    wasmMem.set(input, ptr);
    writeMs += performance.now() - t0;

    t0 = performance.now();
    checksum += BigInt(wasm.bench_sum_bytes(ptr, bytes));
    wasm.bench_fill_bytes(ptr, bytes, i & 0xff);
    computeMs += performance.now() - t0;

    t0 = performance.now();
    const view = wasmMem.subarray(ptr, ptr + bytes);
    checksum += BigInt(view[0] + view[view.length - 1]);
    readMs += performance.now() - t0;
  }

  wasm.bench_dealloc(ptr, bytes);

  const totalBytes = bytes * rounds;
  console.log(JSON.stringify({
    bytesPerRound: bytes,
    rounds,
    totalBytes,
    jsToWasmMs: writeMs,
    wasmToJsMs: readMs,
    wasmComputeMs: computeMs,
    jsToWasmGBs: (totalBytes / (writeMs / 1000)) / (1024 ** 3),
    wasmToJsGBs: (totalBytes / (readMs / 1000)) / (1024 ** 3),
    checksum: checksum.toString(),
  }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
