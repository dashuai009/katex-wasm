# AGENTS.md

This project ports KaTeX JavaScript to a Rust/WASM renderer. The UI must always keep formula previews and browser performance comparisons for three implementations: our `katex-wasm`, `katex.js`, and standalone upstream `katex-rs`.

## Entry and structure

- `src/katex.rs`: `render`, JavaScript `renderToString`, and Rust `render_to_string`.
- `src/Parser.rs`, `src/Lexer.rs`, `src/parse/`: parser; `src/define/macros/`: macros.
- `src/build/`, `src/dom_tree/`, `src/mathML_tree/`: HTML/MathML building and serialization.
- `demo/benchmark.js`: shared settings for all three engines, success-rate precheck, timing scopes, and engine rotation.
- `demo/index.js`, `demo/index.html`: three-column formulas, performance metrics, input preview, and result download.
- `demo/scripts/perf-playwright.mjs`: Chromium production build test.
- `scripts/build-katex-rs.mjs`, `third-party/katex-rs.json`: build upstream source and pin commits. Run `--update` when the user requests the latest version, record the new commit and lock file; do not roll back to older npm packages.

## Build commands

First run in repository root:

```sh
node scripts/build-wasm.mjs
node scripts/build-katex-rs.mjs
cd demo
npm ci
npx playwright install chromium
npm run build:site
npm run serve
```

After changing Rust code, run `npm run build` in `demo` to regenerate WASM and production pages. Reusing old `pkg` or development/profiling builds as release data is forbidden. `npm run build:site` only bundles the existing WASM. Dependency versions are recorded in the root `Cargo.lock`, `demo/package-lock.json`, and `third-party/katex-rs.Cargo.lock`.

CLI for this project (different from third-party `katex-rs`):

```sh
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 5 --summary-only
```

## Correctness rules

This is a **JavaScript-reproducing** project. Before changing Rust logic, read the corresponding `KaTeX/src` code first (initialize the submodule if missing). Use a pinned npm KaTeX for comparison tools, and do not rely on ungenerated `KaTeX/dist`.

After each Rust optimization, rebuild first and then run Diff Harness on a random contiguous block of 50 lines within the first 1000:

```sh
cd demo
npm run test:correctness
# Reproduce already recorded samples; do not select favorable samples:
DIFF_START=100 npm run test:correctness
```

Record sample start positions and results in `demo/perf-results`. Failures must be investigated and not hidden by changing samples. Run the full first 1000 lines if needed:

```sh
node --experimental-wasm-modules tests/diff_harness.mjs tests/fixtures/im2latex_formulas.lst 1 1000 --log-level error
```

HTML attribute order, standalone CSS property order, and decimal format can differ; repeated declarations and overriding shorthand properties must keep their order. Preserve DOM structure, text, attribute semantics, and the existing `0.001em` tolerance. Do not relax tolerance, remove failing formulas, or change benchmark output to pass. A success-rate precheck does not imply semantic parity, and bounded HTML checks in Diff Harness do not imply full KaTeX coverage.

## Performance iteration rules

1. You must compare `renderToString` for all three engines in the **browser** using the same inputs and settings. Fix the dataset to `tests/fixtures/im2latex_formulas.lst` **physical lines 1–1000**.
2. Default is 2 full warm-up rounds plus 6 measured rounds. Do not drop the first few formulas from the statistics. Rotate engine order and do not cache formula outputs. Default run should not enable CPU throttling.
3. In `compute` mode, measure HTML only; in `both` mode, all three engines include the same scope: HTML generation, innerHTML insertion, and forced layout. Do not mix timing scopes across different APIs.
4. Precheck should list each engine’s failures; performance uses the same success set from all three engines. Error marks cannot be counted as success. Do not let third-party or JS initialization cost be timed separately.
5. `npm run perf:playwright` auto-generates `demo/perf-results/im2latex-1-1000-optN.json`, continuing from the highest existing number and never overwriting. The previous final version is the new baseline for the next round.
6. The primary metric is median total batch time; retain all rounds, success set, inputs, WASM hash, build commit, browser, device, and settings. Do not compare across different schemas, inputs, modes, or success sets. Flame graphs must be measured separately; do not use recorded profile runs as final performance evidence.
7. Repeat final performance advantage verification at least once. State exactly which browser, mode, and input set was used. You cannot claim superiority over JS without real measurements, and you cannot present shared-subset advantages as full semantic or all-device superiority.

## GitHub Pages

`.github/workflows/deploy-demo.yml` must continue to automatically build, verify, test, and embed the latest results into static pages. PRs only verify; `main` or manual deployment publishes Pages. Keep repo subpath support, and provide local CSS, fonts, both WASM files, and formula data locally; do not depend on local absolute paths or CDNs.

All commands in `README` must match package scripts and workflow. Do not restore legacy Travis/AppVeyor template flows; they are not used for current Pages publishing.

## Final verification and evidence

- `cargo test --lib`, `npm run test:api`, and `tests/fixtures/performance-regressions.txt` cover allocator, Unicode, macro expansion, settings, style, and trust regressions.
- Final completed performance runs must be revalidated with at least three fresh browser processes using `--require-faster`. Interactive UI tests do not replace these independent measurements.
- Run `npm run test:ui` after Pages build to verify subpath, desktop/mobile layouts, download, and continuous render memory. This test generates extra full-attempt timing for unsupported inputs, which must be distinguished from successful-subset timing.
- `docs/performance.md` and `docs/performance-results/` keep submit-ready baselines, final raw data, and validation summaries; full local iteration history remains in `demo/perf-results/`.
- `src/wasm_allocator.rs` is for non-atomics WASM only; free small-block cache has a fixed upper bound, and you must not switch to bulk arena reset that reclaims still-live objects.

## Demo display and input override (latest user request)

The demo must keep the original plain white, parallel formula layout with the scatter chart before the formula results. Preview and demo/Pages performance input is exclusively `demo/public/formulas.txt`, using the entire file by default. No hardcoded example list or im2latex snapshot may replace it. Retain the newer warmup, timing, rotation and failure-accounting protocol. Demo output names are `formulas-1-<line-count>-optN.json`; do not compare them with historical im2latex reports. The separate 50-line Rust correctness gate remains unchanged.

## Behavior-parity override (latest user request)

Do not use render success/failure as the acceptance criterion for katex-wasm or katex.js. KaTeX.js is the behavior reference: matching normal output or matching error output is acceptable. Compare DOM/text/attributes with the existing numeric tolerance; for exceptions compare category/message, never equate a generic WASM trap with a JS parse error. Only real behavior differences fail the correctness gate. Time every formulas.txt input, including error outcomes, and keep third-party results independent of the input set. Schema 3 replaces the previous successful-subset protocol; reject cross-schema baselines. The UI highlights behavior matches/mismatches, not success rates.

## English HTML-only demo override (latest user request)

All demo interface text must be English. Remove the top toolbar (rerun, mode selector and download). Run automatically, and benchmark HTML generation only; reject other timing modes. The formula source hyperlink points to the GitHub blob page, while actual formula loading remains local `public/formulas.txt` for reproducible Pages builds. Browser checks validate these requirements instead of the removed controls.

The demo must not show the top status text or the Behavior differences disclosure. Keep parity checks and mismatch evidence in the benchmark result and verification gates.
