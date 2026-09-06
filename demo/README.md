# Three-engine browser comparison

Follow the [root README](../README.md) for setup. `npm run serve` starts the local demo; `npm run build` builds release WASM and the static production page. `npm run build:site` reuses the existing WASM package.

## Benchmark

```sh
npm run perf:playwright
npm run perf:playwright -- --baseline perf-results/formulas-1-212-opt0.json --max-regression-pct 10
npm run perf:playwright -- --require-faster
```

The runner serves `dist` on an ephemeral port and launches a fresh Chromium instance. For a `PUBLIC_PATH` subpath build, use `--url` with the address from `npm run preview:pages`, or rebuild without `PUBLIC_PATH` for the automatic server. It does not silently reuse an unrelated development server, copy over input files, or fall back to demo formulas when input loading fails. Run `npm run build` before measuring changed code.

| Option | Default / meaning |
|---|---|
| `--input` | `demo/public/formulas.txt` only |
| `--start`, `--end` | 1, file length; inclusive physical text lines |
| `--repeat`, `--warmup` | 6 measured passes, 2 complete unmeasured passes |
| `--mode` | `compute` only; HTML generation, excluding DOM insertion/layout |
| `--output` | `demo/perf-results/formulas-1-<line-count>-optN.json`, next available iteration |
| `--baseline` | prior compatible schema-3 result; rejects changed input/settings/timed indices/browser |
| `--max-regression-pct` | 10; allowed median batch-time regression |
| `--require-faster` | fail unless ours matches JS behavior and its median beats JS |
| `--cpu-slowdown` | 1 (none); explicit Chromium throttling rate |
| `--cpu-profile` | optional profile file; diagnostic runs must not be used as performance evidence |
| `--url` | explicitly test a running built site, including a Pages subpath |
| `--headless` | true |

Relative file arguments resolve against the current directory; default paths resolve against the project. Historical result files are not overwritten. Sample counts and failure indices remain in JSON.

```sh
npm run perf:playwright -- --cpu-profile perf-results/diagnostic.cpuprofile
```

Open profiles in Chrome DevTools or Speedscope. Unprofiled runs are required for final timing claims. HTML P95 can be noisy due to timer resolution; median full-batch time is the primary comparison.

The UI renders every formula from `public/formulas.txt` in the original plain three-column layout. The scatter chart is above the formulas. It runs the current browser benchmark automatically without a toolbar; no hardcoded examples or other dataset snapshots are displayed.

## Static Pages smoke test

After a benchmark, copy its JSON to `public/latest-benchmark.json`, then:

```sh
PUBLIC_PATH=/katex-wasm/ npm run build:site
npm run test:ui
npm run preview:pages
```

The smoke test checks desktop/mobile previews, the shared font styling, resource paths, the GitHub formula link, HTML-only timing, repeated markup and WASM memory stability. It also records a separate all-input attempt-throughput measurement (including unsupported output). A subpath build is required for this command; set the same `PUBLIC_PATH` on the build and test if the repository name differs.

## Behavior is the correctness criterion

All formulas are timed; errors do not remove inputs. The page reports katex-wasm/JS behavior matches and shows only true differences. Identical returned error markup is valid parity; exception comparisons retain category and message checks, so an unrelated WASM trap cannot masquerade as a matching parse error. Third-party results do not filter the input set. Schema 3 is separate from historical schema-2 successful-subset timings.
