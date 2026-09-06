const fs = require("node:fs");
const path = require("node:path");
const { createHash } = require("node:crypto");
const root = path.resolve(__dirname, "..");
function sourceHash() {
  const files = ["Cargo.toml", "Cargo.lock", "struct_format/Cargo.toml"];
  function visit(dir) {
    for (const entry of fs.readdirSync(path.join(root, dir), {
      withFileTypes: true,
    })) {
      const file = dir + "/" + entry.name;
      if (entry.isDirectory()) visit(file);
      else if (file.endsWith(".rs")) files.push(file);
    }
  }
  visit("src");
  visit("struct_format/src");
  const hash = createHash("sha256");
  for (const file of files.sort())
    hash.update(file).update(fs.readFileSync(path.join(root, file)));
  return hash.digest("hex");
}
function assertFresh() {
  const info = JSON.parse(
    fs.readFileSync(path.join(root, "pkg/build-info.json"), "utf8"),
  );
  if (info.sourceSha256 !== sourceHash())
    throw Error(
      "Rust source changed. Run npm run build:wasm before testing or bundling.",
    );
  const wasm = createHash("sha256")
    .update(fs.readFileSync(path.join(root, "pkg/katex_wasm_bg.wasm")))
    .digest("hex");
  if (wasm !== info.wasmSha256)
    throw Error(
      "WASM was rebuilt outside the release build script. Run npm run build:wasm.",
    );
  return info;
}
module.exports = { sourceHash, assertFresh };
