#!/usr/bin/env node
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = path.join(root, "third-party/katex-rs.json");
const manifest = JSON.parse(await fs.readFile(manifestPath, "utf8"));
const checkout = path.join(root, "third-party/katex-rs-source");
const run = (cmd, args, cwd = checkout) =>
  execFileSync(cmd, args, { cwd, stdio: "inherit" });
try {
  await fs.access(path.join(checkout, ".git"));
} catch {
  await fs.mkdir(checkout, { recursive: true });
  run("git", ["init"]);
  run("git", ["remote", "add", "origin", manifest.repository]);
}
if (process.argv.includes("--update")) {
  manifest.revision = execFileSync(
    "git",
    ["ls-remote", manifest.repository, "HEAD"],
    { encoding: "utf8" },
  ).split(/\s/)[0];
}
run("git", ["fetch", "--depth", "1", "origin", manifest.revision]);
run("git", ["checkout", "--detach", manifest.revision]);
const cargo = await fs.readFile(path.join(checkout, "Cargo.toml"), "utf8");
manifest.version = cargo.match(/^version = "([^"]+)"/m)[1];
// Upstream doesn't commit Cargo.lock. Keep the resolved dependency graph with our manifest.
const lock = path.join(root, "third-party/katex-rs.Cargo.lock");
if (!process.argv.includes("--update")) {
  try {
    await fs.copyFile(lock, path.join(checkout, "Cargo.lock"));
  } catch (e) {
    if (e.code !== "ENOENT") throw e;
  }
}
run("wasm-pack", [
  "build",
  "crates/wasm-binding",
  "--release",
  "--target",
  "bundler",
  "--out-dir",
  "pkg",
]);
await fs.cp(
  path.join(checkout, "crates/wasm-binding/pkg"),
  path.join(root, "third-party/katex-rs-pkg"),
  { recursive: true },
);
await fs.copyFile(
  path.join(checkout, "LICENSE"),
  path.join(root, "third-party/katex-rs-pkg/LICENSE"),
);
await fs.copyFile(
  path.join(checkout, "LICENSE"),
  path.join(root, "third-party/katex-rs.LICENSE"),
);
const pkgPath = path.join(root, "third-party/katex-rs-pkg/package.json");
const pkg = JSON.parse(await fs.readFile(pkgPath, "utf8"));
pkg.license = "MIT";
await fs.writeFile(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
await fs.copyFile(path.join(checkout, "Cargo.lock"), lock);
await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2) + "\n");
