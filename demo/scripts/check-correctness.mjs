import { randomInt } from "node:crypto";
import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";
import state from "../../scripts/build-state.cjs";
state.assertFresh();
const root = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../..",
);
const start = Number(process.env.DIFF_START || randomInt(1, 952));
if (!Number.isInteger(start) || start < 1 || start > 951)
  throw Error("DIFF_START must be 1..951");
const result = spawnSync(
  process.execPath,
  [
    "--experimental-wasm-modules",
    "tests/diff_harness.mjs",
    "tests/fixtures/im2latex_formulas.lst",
    String(start),
    String(start + 49),
    "--log-level",
    "summary",
  ],
  { cwd: root, encoding: "utf8" },
);
const log = `Random contiguous correctness sample: ${start}-${start + 49}\n${result.stdout}${result.stderr}`;
fs.mkdirSync(path.join(root, "demo/perf-results"), { recursive: true });
fs.writeFileSync(
  path.join(root, "demo/perf-results", `correctness-${Date.now()}.log`),
  log,
);
console.log(log);
process.exit(result.status ?? 1);
