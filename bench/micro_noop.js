#!/usr/bin/env node
const path = require('path');
const { pathToFileURL } = require('url');
const { performance } = require('perf_hooks');

const ROOT = path.resolve(__dirname, '..');
const wasmPkgPath = pathToFileURL(path.join(ROOT, 'pkg', 'katex_wasm.js')).href;

async function main() {
  const wasm = await import(wasmPkgPath);
  const totalCalls = Number(process.env.CALLS || 5_000_000);
  const warmup = Number(process.env.WARMUP || 200_000);

  for (let i = 0; i < warmup; i++) wasm.bench_noop();

  const t0 = performance.now();
  for (let i = 0; i < totalCalls; i++) wasm.bench_noop();
  const elapsedMs = performance.now() - t0;

  const nsPerCall = (elapsedMs * 1e6) / totalCalls;
  console.log(JSON.stringify({ totalCalls, elapsedMs, nsPerCall }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
