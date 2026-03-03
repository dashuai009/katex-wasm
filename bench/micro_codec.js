#!/usr/bin/env node
const path = require('path');
const { pathToFileURL } = require('url');
const { performance } = require('perf_hooks');

const ROOT = path.resolve(__dirname, '..');
const wasmPkgPath = pathToFileURL(path.join(ROOT, 'pkg', 'katex_wasm.js')).href;

async function main() {
  const wasm = await import(wasmPkgPath);
  const rounds = Number(process.env.ROUNDS || 25000);
  const payload = process.env.PAYLOAD || String.raw`\\sum_{i=1}^{100}\\frac{i^2}{i+1}+\\sqrt{\\alpha^2+\\beta^2}+\\int_0^1 x^3 dx`;

  for (let i = 0; i < 2000; i++) wasm.bench_echo_string(payload);

  let totalChars = 0;
  const t0 = performance.now();
  for (let i = 0; i < rounds; i++) {
    totalChars += wasm.bench_echo_string(payload).length;
  }
  const elapsedMs = performance.now() - t0;

  console.log(JSON.stringify({
    rounds,
    payloadChars: payload.length,
    totalChars,
    elapsedMs,
    nsPerRound: (elapsedMs * 1e6) / rounds,
    charsPerSecond: totalChars / (elapsedMs / 1000),
  }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
