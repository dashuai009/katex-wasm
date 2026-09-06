# Browser performance and validation — 2026-09-06

Historical schema-2 optimization evidence: these numbers describe the former successful-subset protocol. The current demo uses schema 3, times every `formulas.txt` input, and accepts matching JS errors as behavior parity. Do not use these historical figures as current-demo baselines.


The final implementation is faster than KaTeX.js in three independent Chromium runs on the fixed corpus's common successful subset. The median of those runs is **21.45 ms for ours, 22.90 ms for KaTeX.js, and 37.30 ms for katex-rs**. This is about **1.07× JS throughput** and **1.74× katex-rs throughput**, with roughly **79% less time than our initial 102.3 ms baseline**.

## Reproducible environment

- Apple M5 Max, macOS arm64, Chromium 145.0.7632.6; no CPU throttling or CPU profiling.
- Rust 1.95.0, wasm-pack 0.15.0; our release profile uses optimization level 3, fat LTO and Binaryen `-O4`.
- KaTeX.js is pinned to npm 0.16.29. katex-rs is built from upstream 0.3.0, commit [`f9d93892c39424ba24cef0be18105a1629b66357`](https://github.com/katex-rs/katex-rs/tree/f9d93892c39424ba24cef0be18105a1629b66357), using its own unchanged release profile. Upstream HEAD was rechecked after integration.
- Each engine receives the same `renderToString` settings, including `output: html`, display mode, `trust: false` and `throwOnError: false`. Input is physical lines 1–1000 of `im2latex_formulas.lst`.
- Two complete warmup passes, six measured passes, rotating engine order. The primary measure is the median whole-batch duration. Downloads, module initialization and DOM insertion are excluded from compute mode.
- Build scripts record actual source and WASM hashes. These measurements are from the uncommitted working tree based on `cc4847452e5b5d289c40e5b9c1ade38339467eb7`; the source hash in each final report identifies the code actually built.

## Results and coverage

| Run | Our WASM | KaTeX.js | katex-rs | JS / ours |
|---|---:|---:|---:|---:|
| opt0, initial Rust code | 102.30 ms | 23.05 ms | 36.45 ms | 0.23× |
| opt24, fresh browser | 21.35 ms | 22.90 ms | 37.30 ms | 1.073× |
| opt25, fresh browser | 21.50 ms | 22.90 ms | 37.45 ms | 1.065× |
| opt26, fresh browser | 21.45 ms | 22.70 ms | 37.30 ms | 1.058× |

Medians use the midpoint of the central pair. The historical opt0 aggregate used the upper central value; its table values above are recomputed from its unchanged raw rounds.

**All 1000 inputs are accounted for, but only 397 are successfully rendered by all three engines.** The other 603 produce unsupported/error markup in all three, commonly due to `\label`. Their indices and complete formulas remain in every raw report. Error markup is never counted as a fast successful render. Matching output in the Diff Harness includes matching unsupported output and does not imply that all 1000 formulas are supported.

As a separate check, all 1000 raw inputs were timed without per-call timers, including the 603 unsupported/partial outputs: median **53.95 ms ours, 59.15 ms JS, 96.50 ms katex-rs**, with zero thrown exceptions. This supplementary attempt-throughput measure is recorded separately in [stability.json](performance-results/stability.json); it must not be confused with successful-formula throughput.

The additional **HTML + DOM + synchronous layout** run ([opt27](performance-results/im2latex-1-1000-opt27.json)) measured **110.65 ms ours, 128.60 ms JS, and 140.35 ms katex-rs** on the same 397-formula successful subset. This is a distinct timing scope and must not be compared against compute-mode baselines.

Performance is browser/device dependent. These measurements establish the result on this environment, not on every browser or on first page load. The raw WASM files are 2,593,686 bytes for ours and 796,061 bytes for katex-rs, so warm rendering throughput does not describe download or startup cost.

## Implementation changes

- Share immutable source strings and builtin macro definitions; tokenize constant builtin macro bodies once. Move consumed parse/DOM subtrees instead of deeply copying them, and borrow lookahead tokens when only inspecting them. User macro definitions retain their normal mutable lifetime and randomized maps.
- Use direct ASCII token/symbol paths with the existing Unicode fallback, indexed ASCII font metrics, fixed spacing tables, and fast maps for immutable internal function names. Font metrics values remain unchanged; their generator was updated with the storage layout.
- Stream HTML and escaped CSS into a shared output buffer. Store only populated style fields, keeping the common single-property case inline. Preserve property order, escaping and logical style equality. Use faster numeric formatting with the existing four-decimal rounding behavior.
- Use Talc plus a bounded small-block free list only in non-atomic WASM builds. No live allocations are reset, no formula output is cached, and native/atomic builds retain their usual allocator. The free-list bound is about 3.39 MiB; repeated browser rendering stabilized at **3,473,408 bytes** of total WASM linear memory across eight full-corpus passes.
- Restore omitted `displayMode` defaults, text diaeresis mapping, `\noexpand` token handling, and boolean trust checks for `\href`/`\url` against their JS counterparts. Update the old native smoke test to use the pinned npm reference.

## Validation

- Every Rust iteration passed a random contiguous 50-line Diff Harness sample before performance testing; sample starts and intermediate results are retained locally under `demo/perf-results`.
- Final full-corpus HTML comparison: **1000 passed, 0 mismatched** after canonicalizing attribute/independent-declaration order and using the existing `0.001em` numeric tolerance.
- Additional Unicode, escaping, macro, font, fraction, script and delimiter cases: **15 passed**.
- Native library tests: **7 passed**, including allocator reuse/reallocation/alignment, 100,000 numeric-format comparisons, ASCII-vs-regex tokenization, sparse CSS and spacing-table equivalence.
- WASM API checks cover omitted/mutable options, CSS property aliases, escaping and boolean trust behavior.
- Pages-prefix browser checks cover 15 previews, running the benchmark, downloading its JSON, mobile layout (including expanded failure details), resource loading, repeated output stability and memory stability. No page/resource errors occurred.

The stale-artifact diagnostic opt6 is explicitly invalid and excluded. Profile recordings and intermediate opt3 (which overlapped a native build) are not final performance evidence. Stale artifacts are now rejected by source/WASM fingerprint checks.

## Evidence and rerunning

[Summary](performance-results/summary.json) · [Validation log](performance-results/validation.txt) · [Baseline](performance-results/im2latex-1-1000-opt0.json) · [Final run 1](performance-results/im2latex-1-1000-opt24.json) · [Final run 2](performance-results/im2latex-1-1000-opt25.json) · [Final run 3](performance-results/im2latex-1-1000-opt26.json) · [Desktop](performance-results/desktop.png) · [Mobile](performance-results/mobile.png)

```sh
node scripts/build-wasm.mjs
node scripts/build-katex-rs.mjs
cd demo
npm ci
npx playwright install chromium
npm run test:correctness
npm run test:api
npm run build:site
npm run perf:playwright -- --require-faster
```

For Pages validation, copy a newly produced report to `public/latest-benchmark.json`, build with `PUBLIC_PATH=/katex-wasm/`, then run `npm run test:ui`. CI performs these steps automatically, publishing the actual CI measurement rather than hardcoded local numbers. `npm run preview:pages` opens a local static server for that build.
