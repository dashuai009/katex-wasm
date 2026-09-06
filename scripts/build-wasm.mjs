import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import state from "./build-state.cjs";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const profile = process.argv.includes("--profiling") ? "profiling" : "release";
execFileSync(
  "wasm-pack",
  ["build", `--${profile}`, "--target", "bundler", "--locked"],
  { cwd: root, stdio: "inherit" },
);
const info = {
  profile,
  sourceSha256: state.sourceHash(),
  wasmSha256: createHash("sha256")
    .update(fs.readFileSync(path.join(root, "pkg/katex_wasm_bg.wasm")))
    .digest("hex"),
  revision: execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: root,
    encoding: "utf8",
  }).trim(),
  rustc: execFileSync("rustc", ["--version"], { encoding: "utf8" }).trim(),
};
fs.writeFileSync(
  path.join(root, "pkg/build-info.json"),
  JSON.stringify(info, null, 2) + "\n",
);
