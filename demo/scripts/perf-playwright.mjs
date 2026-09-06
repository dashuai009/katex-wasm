#!/usr/bin/env node
import { median } from "../statistics.mjs";
import buildState from "../../scripts/build-state.cjs";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { serveStatic } from "./static-server.mjs";
import { createHash } from "node:crypto";
import os from "node:os";
import { chromium } from "playwright";
import yaml from "js-yaml";
const demo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const args = {};
for (let i = 2; i < process.argv.length; i++) {
  const key = process.argv[i];
  if (!key.startsWith("--")) throw Error(`Unexpected argument ${key}`);
  args[key.slice(2)] =
    process.argv[i + 1]?.startsWith("--") || !process.argv[i + 1]
      ? "true"
      : process.argv[++i];
}
const supported = new Set([
  "input",
  "start",
  "end",
  "repeat",
  "warmup",
  "mode",
  "output",
  "baseline",
  "max-regression-pct",
  "require-faster",
  "cpu-slowdown",
  "cpu-profile",
  "url",
  "headless",
  "help",
]);
for (const key of Object.keys(args))
  if (!supported.has(key)) throw Error(`Unknown option --${key}`);
if (args.help === "true") {
  console.log(
    "Build first: npm run build. Options: (input: demo/public/formulas.txt) --start 1 --end LINE --repeat 6 --warmup 2 --mode compute --output FILE --baseline FILE --max-regression-pct 10 --require-faster --cpu-profile FILE --cpu-slowdown 1 --url URL",
  );
  process.exit(0);
}
for (const key of ["max-regression-pct", "cpu-slowdown"]) {
  if (
    args[key] !== undefined &&
    (!Number.isFinite(Number(args[key])) ||
      Number(args[key]) < (key === "cpu-slowdown" ? 1 : 0))
  )
    throw Error(`Invalid --${key}`);
}
if (args.mode && args.mode !== "compute")
  throw Error("Only HTML generation (compute) mode is supported");
const expectedBuild = args.url ? null : buildState.assertFresh();
const input = path.resolve(
  args.input || path.join(demo, "public/formulas.txt"),
);
if (input !== path.join(demo, "public/formulas.txt"))
  throw Error("The demo benchmark uses demo/public/formulas.txt only");
const raw = await fs.readFile(input, "utf8");
const formulas = /\.ya?ml$/.test(input)
  ? Object.values(yaml.load(raw))
      .map((v) => (typeof v === "string" ? v : v.tex))
      .filter((v) => typeof v === "string")
  : raw.trimEnd().split("\n");
const start = Number(args.start || 1),
  end = Number(args.end || formulas.length);
if (
  !Number.isInteger(start) ||
  !Number.isInteger(end) ||
  start < 1 ||
  end < start ||
  end > formulas.length
)
  throw Error("Invalid input range");
const resultDir = path.join(demo, "perf-results");
await fs.mkdir(resultDir, { recursive: true });
const files = await fs.readdir(resultDir);
const next =
  Math.max(
    -1,
    ...files.map((f) => Number(f.match(/-opt(\d+)\.json$/)?.[1] ?? -1)),
  ) + 1;
const output = path.resolve(
  args.output ||
    path.join(resultDir, `formulas-${start}-${end}-opt${next}.json`),
);
// Refuse overwriting historical measurements.
try {
  await fs.access(output);
  throw Error(`Result already exists: ${output}`);
} catch (e) {
  if (e.code !== "ENOENT") throw e;
}
let server,
  browser,
  cdp,
  recording = false;
try {
  let base = args.url;
  if (!base) {
    const dist = path.join(demo, "dist");
    await fs.access(path.join(dist, "index.html"));
    ({ server, url: base } = await serveStatic(dist));
  }
  browser = await chromium.launch({ headless: args.headless !== "false" });
  const page = await browser.newPage();

  cdp = await page.context().newCDPSession(page);
  const slowdown = Number(args["cpu-slowdown"] || 1);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: slowdown });
  const url = new URL(base);
  url.searchParams.set("autorun", "0");
  for (const [key, value] of Object.entries({
    start,
    end,
    repeat: args.repeat || 6,
    warmup: args.warmup ?? 2,
    mode: args.mode || "compute",
  }))
    url.searchParams.set(key, String(value));
  await page.goto(url.toString());
  await page.waitForFunction(
    () => window.__RUN_BENCHMARK__ || window.__PERF_RESULT__?.error,
    null,
    { timeout: 180000 },
  );
  if (args["cpu-profile"]) {
    await cdp.send("Profiler.enable");
    await cdp.send("Profiler.setSamplingInterval", { interval: 100 });
    await cdp.send("Profiler.start");
    recording = true;
  }
  const result = await page.evaluate(async () => {
    if (!window.__RUN_BENCHMARK__) throw Error(window.__PERF_RESULT__.error);
    await window.__RUN_BENCHMARK__();
    return window.__PERF_RESULT__;
  });
  if (result.error) throw Error(result.error);
  if (
    expectedBuild &&
    (expectedBuild.sourceSha256 !== result.build.sourceSha256 ||
      expectedBuild.wasmSha256 !== result.build.wasmSha256 ||
      expectedBuild.profile !== result.build.profile)
  )
    throw Error("dist contains stale WASM. Run npm run build:site.");
  result.environment = {
    ...result.environment,
    browser: browser.version(),
    platform: os.platform(),
    arch: os.arch(),
    cpu: os.cpus()[0]?.model,
    cpuSlowdown: slowdown,
    profiled: Boolean(args["cpu-profile"]),
  };
  result.input = {
    file: path.relative(path.join(demo, ".."), input),
    sha256: createHash("sha256")
      .update(formulas.slice(start - 1, end).join("\n"))
      .digest("hex"),
  };

  await fs.mkdir(path.dirname(output), { recursive: true });
  await fs.writeFile(output, JSON.stringify(result, null, 2) + "\n");
  console.log(`Result: ${output}`);
  console.log(
    `Behavior parity: ${result.behavior.matchedCount}/${result.inputCount}; timed inputs: ${result.timedCount}`,
  );
  for (const e of result.engines)
    console.log(
      `${e.name}: median batch ${result.aggregateSummary[e.id].batch.median.toFixed(3)} ms; thrown exceptions ${result.exceptions[e.id].length}`,
    );
  console.log(
    `JS / ours: ${result.aggregateSummary.ratio.meanJsDivWasm.toFixed(3)}x`,
  );
  if (result.behavior.mismatches.length)
    throw Error(
      `katex-wasm differs from JS on ${result.behavior.mismatches.length} inputs; see result JSON`,
    );
  if (
    args["require-faster"] === "true" &&
    (result.environment.profiled ||
      result.build.profile !== "release" ||
      result.behavior.mismatches.length > 0 ||
      result.aggregateSummary.ratio.meanJsDivWasm <= 1)
  )
    throw Error("Our engine must match JS behavior and beat JS");
  if (args.baseline) {
    const baseline = JSON.parse(
      await fs.readFile(path.resolve(args.baseline), "utf8"),
    );
    if (
      baseline.invalidReason ||
      baseline.environment.profiled ||
      result.environment.profiled
    )
      throw Error("Invalid or profiled runs cannot be performance baselines");
    for (const key of [
      "schemaVersion",
      "input",
      "config",
      "timedIndices",
      "engines",
    ])
      if (JSON.stringify(result[key]) !== JSON.stringify(baseline[key]))
        throw Error(`Incompatible baseline: ${key}`);
    if (
      result.environment.browser !== baseline.environment.browser ||
      result.environment.cpuSlowdown !== baseline.environment.cpuSlowdown
    )
      throw Error("Incompatible browser / slowdown");
    const pct =
      100 *
      (result.aggregateSummary.wasm.batch.median /
        median(baseline.rounds.map((round) => round.wasm)) -
        1);
    if (pct > Number(args["max-regression-pct"] || 10))
      throw Error(`Regression: ${pct.toFixed(2)}%`);
  }
} finally {
  if (recording && cdp) {
    const { profile } = await cdp.send("Profiler.stop");
    await fs.mkdir(path.dirname(path.resolve(args["cpu-profile"])), {
      recursive: true,
    });
    await fs.writeFile(
      path.resolve(args["cpu-profile"]),
      JSON.stringify(profile),
    );
  }
  await browser?.close();
  await new Promise((r) => (server ? server.close(r) : r()));
}
