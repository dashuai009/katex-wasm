import { outcome, compareOutcomes } from "./behavior.mjs";
import { stats } from "./statistics.mjs";
import * as ours from "katex-wasm";
import { memory as ourMemory } from "../pkg/katex_wasm_bg.wasm";
export const memoryBytes = () => ourMemory.buffer.byteLength;
import * as upstream from "katex-rs";
import katex from "katex";
import upstreamInfo from "../third-party/katex-rs.json";
export const engines = [
  {
    id: "wasm",
    name: "katex-wasm (ours)",
    version: "0.1.0",
    render: ours.renderToString,
  },
  {
    id: "katex",
    name: "katex.js",
    version: "0.16.29",
    render: katex.renderToString,
  },
  {
    id: "katexRs",
    name: "katex-rs",
    version: upstreamInfo.version,
    revision: upstreamInfo.revision,
    render: upstream.renderToString,
  },
];
export const settings = {
  displayMode: true,
  output: "html",
  throwOnError: false,
  strict: "ignore",
  trust: false,
  maxExpand: 1000,
  maxSize: 200000,
  errorColor: "#b00001",
};
export function render(engine, formula) {
  return engine.render(formula, { ...settings });
}
const pause = () => new Promise((resolve) => setTimeout(resolve, 0));
export async function benchmark(
  formulas,
  { repeat = 6, warmup = 2, mode = "compute" } = {},
  progress = () => {},
) {
  if (mode !== "compute")
    throw new Error("Only HTML generation benchmarks are supported");
  if (!formulas.length) throw new Error("No formulas to benchmark");
  const exceptions = Object.fromEntries(engines.map((e) => [e.id, []]));
  const checks = formulas.map((formula, index) => {
    const results = Object.fromEntries(
      engines.map((engine) => [
        engine.id,
        outcome(engine.render, formula, settings),
      ]),
    );
    for (const engine of engines)
      if (results[engine.id].kind === "throw")
        exceptions[engine.id].push({
          index,
          formula,
          error: results[engine.id].message,
        });
    const comparison = compareOutcomes(results.katex, results.wasm);
    return {
      index,
      formula,
      ...comparison,
      ...(!comparison.matched
        ? { reference: results.katex, actual: results.wasm }
        : {}),
    };
  });
  const behavior = {
    matchedCount: checks.filter((c) => c.matched).length,
    total: formulas.length,
    mismatches: checks.filter((c) => !c.matched),
    checks,
  };
  // Time every input. Matching error outputs/exceptions are valid parity cases.
  const common = formulas.map((_, i) => i);
  const samples = Object.fromEntries(engines.map((e) => [e.id, []]));
  const rounds = [];
  let checksum = 0;
  function batch(engine, measured) {
    const values = [];
    const start = performance.now();
    for (const i of common) {
      const t = performance.now();
      try {
        const html = engine.render(formulas[i], { ...settings });
        checksum = (checksum + html.length) >>> 0;
      } catch (error) {
        checksum = (checksum + String(error).length) >>> 0;
      }
      if (measured) values.push(performance.now() - t);
    }
    const total = performance.now() - start;
    if (measured) samples[engine.id].push(...values);
    return total;
  }
  {
    for (let r = 0; r < warmup + repeat; r++) {
      const row = {};
      const order = engines.map((_, i) => engines[(i + r) % 3]);
      progress(
        `${r < warmup ? "Warm-up" : "Measurement"} ${r < warmup ? r + 1 : r - warmup + 1}/${r < warmup ? warmup : repeat}`,
      );
      for (const engine of order) {
        await pause();
        row[engine.id] = batch(engine, r >= warmup);
      }
      if (r >= warmup) rounds.push(row);
    }
  }
  const aggregateSummary = { sampleCount: common.length * repeat };
  for (const engine of engines)
    aggregateSummary[engine.id] = {
      ...stats(samples[engine.id]),
      batch: stats(rounds.map((r) => r[engine.id])),
    };
  aggregateSummary.ratio = {
    meanJsDivWasm:
      aggregateSummary.katex.batch.median / aggregateSummary.wasm.batch.median,
  };
  return {
    schemaVersion: 3,
    build: __BUILD_INFO__,
    timestamp: new Date().toISOString(),
    config: { repeat, warmup, mode, settings },
    engines: engines.map(({ render, ...e }) => e),
    environment: {
      wasmMemoryBytes: memoryBytes(),
      userAgent: navigator.userAgent,
      hardwareConcurrency: navigator.hardwareConcurrency,
    },
    inputCount: formulas.length,
    timedCount: common.length,
    timedIndices: common,
    behavior,
    exceptions,
    rounds,
    aggregateSummary,
    perFormula: common.map((index, position) => ({
      index,
      formula: formulas[index],
      times: Object.fromEntries(
        engines.map((engine) => [
          engine.id,
          stats(
            Array.from(
              { length: repeat },
              (_, round) =>
                samples[engine.id][round * common.length + position],
            ),
          ),
        ]),
      ),
    })),
    checksum,
  };
}
