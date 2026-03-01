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
```

Arguments:

- `formulas.txt`: input file containing one LaTeX formula per line
- `start_line`: optional 1-based inclusive start line, defaults to `1`
- `end_line`: optional 1-based inclusive end line, defaults to the last line in the file

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

