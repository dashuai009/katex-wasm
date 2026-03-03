#!/usr/bin/env node
const fs = require('fs');
const os = require('os');
const path = require('path');
const { pathToFileURL } = require('url');
const { performance } = require('perf_hooks');

const ROOT = path.resolve(__dirname, '..');
const wasmPkgPath = pathToFileURL(path.join(ROOT, 'pkg', 'katex_wasm.js')).href;

const SIZES = {
  small: [
    'E=mc^2',
    String.raw`\\frac{a}{b}+\\sqrt{x}`,
    String.raw`\\int_0^1 x^2 dx`,
  ],
  medium: [
    String.raw`\\sum_{i=1}^{n} i^2 + \\prod_{j=1}^{m} (1+x_j)`,
    String.raw`\\left(\\frac{\\alpha+\\beta}{\\gamma}\\right)^{2k} + \\sqrt{1+\\frac{1}{n}}`,
    String.raw`\\begin{aligned}a&=b+c\\\\d&=e+f\\\\g&=h+i\\end{aligned}`,
  ],
  large: [
    String.raw`\\left(\\sum_{i=1}^{100}\\frac{i^2}{i+1}\\right)+\\left(\\prod_{k=1}^{30}(1+\\frac{1}{k})\\right)+\\int_{0}^{\\pi}\\sin(x)^2dx`,
    String.raw`\\begin{aligned}f(x)&=\\sum_{n=0}^{20}\\frac{(-1)^n x^{2n+1}}{(2n+1)!}\\\\g(x)&=\\sum_{n=0}^{15}\\frac{x^n}{n!}\\\\h(x)&=\\prod_{k=1}^{20}\\left(1+\\frac{x}{k}\\right)\\end{aligned}`,
    String.raw`\\left[\\begin{array}{cccc}1&2&3&4\\\\5&6&7&8\\\\9&10&11&12\\\\13&14&15&16\\end{array}\\right]\\cdot\\left(\\begin{array}{c}x\\\\y\\\\z\\\\w\\end{array}\\right)`,
  ],
};

const SETTINGS = {
  displayMode: true,
  output: 'html',
  throwOnError: false,
  strict: 'ignore',
  trust: true,
  maxSize: 200000,
  maxExpand: 1000,
};

const CONFIG = {
  warmupRounds: Number(process.env.WARMUP_ROUNDS || 8),
  measureRounds: Number(process.env.MEASURE_ROUNDS || 20),
  innerLoops: Number(process.env.INNER_LOOPS || 50),
  forceGcBetweenRounds: process.env.FORCE_GC === '1',
};

function percentile(sorted, p) {
  if (!sorted.length) return 0;
  const idx = Math.min(sorted.length - 1, Math.ceil((p / 100) * sorted.length) - 1);
  return sorted[idx];
}

function summarize(samplesMs) {
  const sorted = [...samplesMs].sort((a, b) => a - b);
  const sum = sorted.reduce((acc, cur) => acc + cur, 0);
  const meanMs = sum / sorted.length;
  return {
    meanMs,
    p50Ms: percentile(sorted, 50),
    p95Ms: percentile(sorted, 95),
    p99Ms: percentile(sorted, 99),
  };
}

async function main() {
  const wasm = await import(wasmPkgPath);
  const cases = Object.entries(SIZES);

  const envInfo = {
    node: process.version,
    platform: `${os.platform()} ${os.release()} ${os.arch()}`,
    cpu: os.cpus()[0]?.model || 'unknown',
    wasmBuild: process.env.WASM_BUILD || 'release (recommended)',
    config: CONFIG,
  };

  console.log('# katex-wasm benchmark');
  console.log(JSON.stringify(envInfo, null, 2));

  const results = [];
  for (const [size, formulas] of cases) {
    const itemsPerRound = formulas.length * CONFIG.innerLoops;

    for (let i = 0; i < CONFIG.warmupRounds; i++) {
      for (let loop = 0; loop < CONFIG.innerLoops; loop++) {
        for (const f of formulas) {
          wasm.renderToString(f, SETTINGS);
        }
      }
    }

    const samples = [];
    for (let round = 0; round < CONFIG.measureRounds; round++) {
      if (CONFIG.forceGcBetweenRounds && global.gc) global.gc();
      const t0 = performance.now();
      for (let loop = 0; loop < CONFIG.innerLoops; loop++) {
        for (const f of formulas) {
          wasm.renderToString(f, SETTINGS);
        }
      }
      const elapsed = performance.now() - t0;
      samples.push(elapsed);
    }

    const stats = summarize(samples);
    const qps = itemsPerRound / (stats.meanMs / 1000);
    const mem = process.memoryUsage();

    results.push({
      case: size,
      formulas: formulas.length,
      itemsPerRound,
      ...stats,
      qps,
      rssMB: mem.rss / 1024 / 1024,
      heapUsedMB: mem.heapUsed / 1024 / 1024,
    });
  }

  console.table(results.map((r) => ({
    case: r.case,
    mean_ms: r.meanMs.toFixed(2),
    p50_ms: r.p50Ms.toFixed(2),
    p95_ms: r.p95Ms.toFixed(2),
    p99_ms: r.p99Ms.toFixed(2),
    qps: Math.round(r.qps),
    rss_mb: r.rssMB.toFixed(1),
    heap_used_mb: r.heapUsedMB.toFixed(1),
  })));

  fs.writeFileSync(path.join(ROOT, 'profiles', 'bench-last.json'), JSON.stringify({ envInfo, results }, null, 2));
  console.log('Saved: profiles/bench-last.json');
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
