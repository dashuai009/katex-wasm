# Demo 开发与部署

## 本地开发

```bash
# 在仓库根目录构建 wasm 包（输出到 ../pkg）
wasm-pack build

# 启动 demo
cd demo
npm install
npm run serve
```

> `demo` 依赖 `file:../pkg`，因此每次修改 Rust 代码后都需要重新执行一次 `wasm-pack build`。
>
> `webpack.config.j` 现在仅通过环境变量 `WASM_PROFILE` 决定 wasm profile：
> - `WASM_PROFILE=dev` -> `wasm-pack build --dev`
> - `WASM_PROFILE=release` -> `wasm-pack build --release`
> - `WASM_PROFILE=profiling` -> `wasm-pack build --profile profiling`

## GitHub Pages 构建

GitHub Pages 通常部署在 `https://<owner>.github.io/<repo>/`，因此 webpack 的 `publicPath` 必须指向仓库子路径，否则会出现 JS/WASM 资源 404。

`webpack.config.js` 已支持通过环境变量控制：

- `PUBLIC_PATH`：静态资源前缀（例如 `/katex-wasm/`）
- `NODE_ENV=production`：生产构建

示例：

```bash
cd demo
PUBLIC_PATH=/katex-wasm/ npm run build
```

## Playwright 自动化性能测试

已提供自动化脚本：`demo/scripts/perf-playwright.mjs`。

### 1) 安装依赖

```bash
cd demo
npm install
```

### 2) 一键执行（自动启动 demo，自动打开页面，自动导出 JSON）

```bash
cd demo
npm run perf:playwright -- \
  --input ../tests/fixtures/im2latex_formulas.lst \
  --start 1 \
  --end 500 \
  --repeat 3 \
  --warmup 1 \
  --mode compute \
  --output ../demo/perf-results/im2latex-1-500.json
```

说明：

- `--input`：本地公式文件路径；支持逐行文本文件，也支持 `.yml/.yaml`，脚本会先复制到 `demo/public/`，再由 demo 页面按文件类型解析
- `--start/--end`：行号范围（1-based）
- `--repeat`：重复轮次
- `--warmup`：每轮前 N 条样本不计入统计
- `--mode`：`compute`（仅渲染计算）或 `both`（渲染 + DOM）
- `--output`：结果 JSON 输出路径
- `--cpu-profile`：可选，输出 Chromium CPU profile（`.cpuprofile`），用于火焰图分析
- `--cpu-sampling-interval-us`：可选，CPU 采样间隔（微秒），默认 `100`

默认不传 `--input` 时，行为保持不变，仍然使用 `demo/public/formulas.txt`。

当 `--input` 指向 `KaTeX/test/screenshotter/ss_data.yaml` 这类 YAML 文件时，demo 页面会按条目提取公式：

- 值为字符串的条目，直接取该字符串
- 值为对象且包含 `tex` 字段的条目，取 `tex`
- YAML 中的 `display`、`macros`、`pre/post`、`styles` 等 option 不参与 perf 测试
- `--start/--end` 对 YAML 输入按“提取后的公式顺序”生效，不按文件物理行号生效

示例：

```bash
cd demo
npm run perf:playwright -- \
  --input ../KaTeX/test/screenshotter/ss_data.yaml \
  --start 1 \
  --end 50 \
  --repeat 3 \
  --warmup 1 \
  --mode compute
```

### 2.1) 导出火焰图数据（CPU Profile）

快捷命令（默认输出到 `../demo/perf-results/perf.cpuprofile`）：

```bash
cd demo
npm run perf:playwright:flame -- --start 1 --end 500 --repeat 3 --warmup 1 --mode compute
```

```bash
cd demo
npm run perf:playwright -- \
  --input ../tests/fixtures/im2latex_formulas.lst \
  --start 1 \
  --end 500 \
  --repeat 3 \
  --warmup 1 \
  --mode compute \
  --output ../demo/perf-results/im2latex-1-500.json \
  --cpu-profile ../demo/perf-results/im2latex-1-500.cpuprofile
```

输出的 `.cpuprofile` 可以直接用于火焰图查看：

- Chrome DevTools `Performance` 面板加载 profile 文件
- 或拖到 https://www.speedscope.app/ 进行交互式火焰图分析

### 3) 阈值与基线回归（可用于 CI）

```bash
cd demo
npm run perf:playwright -- \
  --input ../tests/fixtures/im2latex_formulas.lst \
  --start 1 --end 500 --repeat 3 --warmup 1 --mode compute \
  --baseline ../demo/perf-results/baseline.json \
  --max-regression-pct 10 \
  --max-wasm-p95-ms 3.5
```

当阈值超限时脚本会以非 0 退出码结束，可直接接 CI gate。
