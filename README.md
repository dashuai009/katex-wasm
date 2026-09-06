# katex-wasm

A Rust / WebAssembly port of KaTeX, with a browser comparison of **our katex-wasm**, **KaTeX.js 0.16.29**, and **[katex-rs 0.3.0](https://github.com/katex-rs/katex-rs)**. The third engine uses upstream HEAD verified on 2026-09-06, pinned to commit `f9d93892c39424ba24cef0be18105a1629b66357` for reproducibility. It is unrelated to this project's similarly named CLI.

The demo keeps the original plain white, side-by-side formula layout with a three-engine scatter chart at the top. Both previews and automatic browser measurements read all formulas directly from `demo/public/formulas.txt`; edit this file to change the demo. GitHub Pages builds the same interface automatically.

## Build and run

Requires Rust stable with `wasm32-unknown-unknown`, Node.js 24, npm, and wasm-pack 0.15.0.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
node scripts/build-wasm.mjs
node scripts/build-katex-rs.mjs
cd demo
npm ci
npm run serve
```

After changing Rust, run `npm run build` from `demo` to rebuild release WASM and the static page. `npm run build:site` only rebuilds the JavaScript/static bundle using the existing `pkg`. Development serving also uses production JavaScript; benchmark only the built `dist` output.

## Browser performance workflow

```sh
cd demo
npx playwright install chromium
npm run test:correctness
npm run build:site
npm run perf:playwright
```

Defaults: **all physical lines** of `demo/public/formulas.txt`, **2 full warmup passes**, **6 measured passes**, HTML output, no CPU throttling, fresh headless Chromium. Each renderer uses `renderToString` with identical options. Engine order rotates between passes. Warmup never removes measured formulas; no rendered-formula cache is used.

To refresh katex-rs to the current upstream HEAD, run `npm run update:katex-rs` from `demo`, then rebuild the page and establish a new baseline. The source commit, version and dependency lock are tracked in `third-party/`. Ordinary builds use that recorded revision.

Results are written to `demo/perf-results/formulas-1-<line-count>-optN.json`, automatically continuing the largest existing `optN`. Existing files cannot be overwritten. Use the last completed iteration as the next baseline:

```sh
npm run perf:playwright -- --baseline perf-results/formulas-1-212-opt0.json
npm run perf:playwright -- --require-faster
```

The primary metric is median total time for **all inputs**, including error output and thrown exceptions. Correctness means **our behavior matches KaTeX.js**, not that either renderer supports every formula. Matching error HTML is a match; matching exceptions are a match after normalizing the KaTeX/ParseError prefix, while different exception types/messages or return-vs-throw behavior are mismatches. HTML comparison retains DOM/text/attribute semantics and the existing `0.001em` tolerance. katex-rs remains an independent comparator and does not determine which inputs are timed.

JSON includes behavior matches and mismatch evidence, all timed indices and rounds, per-formula timing, exception diagnostics and build/environment/input fingerprints. A normal benchmark fails its verification gate when ours differs from JS. `--require-faster` additionally requires an unprofiled release build to beat JS; matching JS errors never fail that gate.

Schema **3** records all-input attempt timings and behavior parity. It cannot be compared directly with the old schema-2 successful-subset results. The current benchmark measures HTML generation only; DOM insertion and layout are not timed. Other timing modes are rejected.

See [demo/README.md](demo/README.md) for profiling and exact options. Historical im2latex measurements use a different input set and must not be compared with the current demo. Performance evidence and optimization notes are in [docs/performance.md](docs/performance.md).

## Correctness and JS reference

Before changing Rust rendering logic, inspect its counterpart in `KaTeX/src`. Hydrate this reference when needed with `git submodule update --init --recursive`. Runtime comparison uses the pinned npm KaTeX package, so it needs no local KaTeX distribution build.

The build script records source and WASM fingerprints in `pkg/build-info.json`. Correctness and bundling reject stale or externally replaced artifacts. Use `node scripts/build-wasm.mjs --profiling` only for diagnostic profiles, then rebuild release before timing.

Every Rust optimization must first pass a random contiguous 50-formula sample from the first 1000 lines:

```sh
cd demo
npm run test:correctness
# Reproduce a recorded sample:
DIFF_START=100 npm run test:correctness
```

The harness compares HTML with its existing numeric tolerance after canonicalizing HTML attributes and independent CSS declarations. Duplicate/overlapping declarations retain their order. It is a bounded rendering check, not proof of complete KaTeX compatibility. For an explicit range:

```sh
node --experimental-wasm-modules tests/diff_harness.mjs tests/fixtures/im2latex_formulas.lst 1 1000 --log-level summary
```

It also supports YAML mappings with string or `tex` values. YAML options are not imported.

## APIs and CLI

- `render(expression, base_node, options)` renders to a browser DOM node.
- JavaScript `renderToString(expression, settings)` returns markup.
- Rust `render_to_string(expression, settings)` returns markup natively.

```sh
cargo run --bin katex-rs-cli -- --help
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 5 --summary-only
```

The local CLI belongs to **this project**. It is unrelated to the third-party katex-rs package in the demo.

## GitHub Pages

[The workflow](.github/workflows/deploy-demo.yml) runs on pushes to `main`, pull requests, and manual dispatch. It builds release WASM, installs locked npm dependencies, runs the correctness sample and three-engine browser benchmark, embeds the measured JSON in the static site, and deploys Pages on main/manual runs. CI reports use the workflow run number in their `ci-optN.json` filename. Pull requests validate without deploying. Set the repository's Pages source to **GitHub Actions**.

The final build uses `PUBLIC_PATH=/<repository>/`; all WASM, CSS, fonts and formula data are bundled locally. No CDN or application server is required. To build the same subpath locally:

```sh
cd demo
PUBLIC_PATH=/katex-wasm/ npm run build:site
```

## Source layout

`src/parse`, `src/Parser.rs`, `src/Lexer.rs` parse LaTeX; `src/build` builds HTML/MathML; `src/dom_tree` serializes DOM nodes; `src/metrics` supplies font metrics. `src/katex.rs` provides the public rendering entry points. See [AGENTS.md](AGENTS.md) for maintenance rules.
