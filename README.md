<div align="center">

  <h1><code>katex-wasm</code></h1>

  <strong>A Rust version of Katex using wasm. <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

## About

katex-wasm

## 🚴 develop

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

Use the custom Cargo profile defined in `Cargo.toml` (`[profile.profiling]`):

```bash
wasm-pack build --profile profiling
```

Note: `wasm-pack build --profiling` is wasm-pack's built-in profiling mode and does not select your custom `[profile.profiling]`.

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

## 🔋 demo

```
wasm-pack build
cd demo
npm install
npm start
```

## CLI

`katex-rs-cli` now uses `clap` for argument parsing, so it has standard `--help` and `--version` output.

```bash
# Show help
cargo run --bin katex-rs-cli -- --help

# Render every formula in a file
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt

# Render a specific inclusive line range
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 5

# Only print the final summary
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 5 --summary-only

# Disable multi-threaded rendering
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 5 --multi-threaded false
```

Arguments:

- `formulas.txt`: input file containing one LaTeX formula per line
- `start_line`: optional 1-based inclusive start line, defaults to `1`
- `end_line`: optional 1-based inclusive end line, defaults to the last line in the file
- `--summary-only`: optional flag that suppresses per-formula output and only prints the final summary
- `--multi-threaded <BOOL>`: optional boolean switch for parallel rendering, defaults to `true`

Coverage testing for the CLI is documented in [docs/katex-rs-cli-coverage.md](/home/dashuai/katex-wasm/docs/katex-rs-cli-coverage.md).

Quick coverage run with a `.txt` formula file:

```bash
scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas.txt
```

The coverage script uses `katex-rs-cli --summary-only` to print only the final summary.

Merge coverage from two formula files:

```bash
scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas_part1.txt --profraw-dir coverage/profraw/part1
scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas_part2.txt --profraw-dir coverage/profraw/part2 --merge-profraw coverage/profraw/part1
```

## Diff Harness

Use `tests/diff_harness.mjs` to compare JS KaTeX and Rust WASM output on the same formulas.

```bash
wasm-pack build
node --experimental-wasm-modules tests/diff_harness.mjs tests/fixtures/formulas.txt 1 5 --log-level error
```

The harness now accepts either:

- text input with one formula per line
- YAML input such as `KaTeX/test/screenshotter/ss_data.yaml`; it extracts formulas from string values or object `tex` fields

For YAML input, `start_line` and `end_line` apply to the extracted formula order, not physical file lines.

```bash
wasm-pack build
node --experimental-wasm-modules tests/diff_harness.mjs KaTeX/test/screenshotter/ss_data.yaml 1 20 --log-level summary
```

Log levels:

- `summary`: only the final summary
- `error`: only render errors or mismatches
- `normal`: per-formula render results and match status
- `debug`: `normal` plus processed JS/Rust settings

YAML parsing in the harness uses `demo`'s `js-yaml` dependency, so make sure `cd demo && npm install` has been run at least once before using YAML input.

## 🚀 Deploy demo to GitHub Pages

已经提供自动化工作流：`.github/workflows/deploy-demo.yml`。

### 触发方式

- 推送到 `main` 分支自动部署
- 或在 GitHub Actions 页面手动触发 `workflow_dispatch`

### 流程说明

1. `wasm-pack build` 在仓库根目录构建 wasm，产物默认输出到 `pkg/`
2. `demo/package.json` 使用 `"katex-wasm": "file:../pkg"`，直接消费该产物
3. 构建 demo 时通过 `PUBLIC_PATH=/${{ github.event.repository.name }}/` 注入 webpack `output.publicPath`
4. 将 `demo/dist` 上传到 GitHub Pages 并发布

> 注意：GitHub Pages 项目页不是根路径（`/`），如果不设置 `publicPath`，`index.js` 和 wasm 文件会按根路径请求，导致 404。

