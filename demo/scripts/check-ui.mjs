import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { serveStatic } from "./static-server.mjs";
const demo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const output = path.join(demo, "perf-results/ui");
await fs.mkdir(output, { recursive: true });
const { server, url } = await serveStatic(
  path.join(demo, "dist"),
  process.env.PUBLIC_PATH || "/katex-wasm/",
);
let browser;
try {
  browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1440, height: 1000 },
  });
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("response", (response) => {
    if (response.status() >= 400 && !response.url().endsWith("/favicon.ico"))
      errors.push(`${response.status()} ${response.url()}`);
  });
  await page.goto(url, { waitUntil: "networkidle" });
  await page.waitForFunction(() => window.__RUN_BENCHMARK__);
  await page.waitForFunction(() => window.__PERF_DONE__ === true, null, {
    timeout: 180000,
  });
  const inputFormulas = (
    await fs.readFile(path.join(demo, "public/formulas.txt"), "utf8")
  )
    .trimEnd()
    .split("\n");
  assert.equal(await page.locator("#summary tbody tr").count(), 3);
  assert.equal(
    await page.locator(".formula").count(),
    inputFormulas.length * 3,
  );
  assert.ok(
    await page.evaluate(
      () =>
        document.getElementById("chart-container").getBoundingClientRect().top <
        document.getElementById("formulas").getBoundingClientRect().top,
    ),
  );
  assert.equal(
    await page.locator(".controls, #run, #mode, #download").count(),
    0,
  );
  assert.equal(await page.locator("html").getAttribute("lang"), "en");
  assert.equal(await page.locator("#status, details").count(), 0);
  assert.equal(
    await page.locator("a").first().getAttribute("href"),
    "https://github.com/dashuai009/katex-wasm/blob/main/demo/public/formulas.txt",
  );
  const result = await page.evaluate(() => window.__PERF_RESULT__);
  assert.equal(result.inputCount, inputFormulas.length);
  assert.equal(result.engines.length, 3);
  assert.equal(result.build.profile, "release");
  assert.equal(result.timedCount, inputFormulas.length);
  assert.equal(result.perFormula.length, inputFormulas.length);
  assert.equal(result.behavior.mismatches.length, 0);
  assert.equal(result.config.mode, "compute");
  await fs.writeFile(
    path.join(output, "result.json"),
    JSON.stringify(result, null, 2),
  );
  await page.screenshot({
    path: path.join(output, "desktop.png"),
    fullPage: false,
  });
  const formulas = inputFormulas;
  const stability = await page.evaluate(
    ({ formulas }) => {
      const engine = window.__ENGINES__[0],
        settings = window.__PERF_RESULT__.config.settings;
      const snapshot = (formula) => {
        try {
          return JSON.stringify({
            kind: "return",
            html: engine.render(formula, { ...settings }),
          });
        } catch (error) {
          return JSON.stringify({
            kind: "throw",
            name: error?.name,
            message: error?.message || String(error),
          });
        }
      };
      const reference = formulas.map(snapshot);
      const memory = [];
      for (let pass = 0; pass < 8; pass++) {
        for (let i = 0; i < formulas.length; i++)
          if (snapshot(formulas[i]) !== reference[i])
            throw Error(`Unstable markup at line ${i + 1}`);
        memory.push(window.__WASM_MEMORY_BYTES__());
      }
      const rounds = [];
      let checksum = 0;
      for (let r = 0; r < 8; r++) {
        const row = {};
        for (let i = 0; i < 3; i++) {
          const engine = window.__ENGINES__[(r + i) % 3];
          const start = performance.now();
          let throws = 0;
          for (const formula of formulas) {
            try {
              checksum += engine.render(formula, { ...settings }).length;
            } catch {
              throws++;
            }
          }
          row[engine.id] = { ms: performance.now() - start, throws };
        }
        if (r >= 2) rounds.push(row);
      }
      return {
        memory,
        allInputAttempts: {
          description:
            "All formulas.txt inputs, including unsupported/error markup. Not a success-only comparison; no per-call timer.",
          warmup: 2,
          repeat: 6,
          rounds,
          checksum,
        },
        environment: window.__PERF_RESULT__.environment,
      };
    },
    { formulas },
  );
  assert.ok(
    stability.memory.at(-1) <= stability.memory[2] + 65536,
    "WASM memory must stabilize after repeated renders",
  );
  await fs.writeFile(
    path.join(output, "stability.json"),
    JSON.stringify(stability, null, 2),
  );
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForFunction(
    () => document.documentElement.scrollWidth <= innerWidth,
    null,
    { timeout: 5000 },
  );
  assert.ok(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
    "Mobile page must not overflow horizontally",
  );
  await page.screenshot({
    path: path.join(output, "mobile.png"),
    fullPage: false,
  });
  assert.deepEqual(errors, []);
  console.log(
    JSON.stringify(
      {
        url,
        previewCount: inputFormulas.length * 3,
        behaviorMatches: result.behavior.matchedCount,
        memory: stability.memory,
        errors,
      },
      null,
      2,
    ),
  );
} finally {
  await browser?.close();
  await new Promise((resolve) => server.close(resolve));
}
