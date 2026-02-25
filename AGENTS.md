# AGENTS.md

本项目为 KaTeX 的 Rust WASM 移植版本，用于在 Web 浏览器中渲染 LaTeX 数学公式。

## 项目概述

katex-wasm 是将 JavaScript 版本的 KaTeX 移植到 Rust WebAssembly 的项目。它允许使用 Rust 生成的 WASM 代码替代 JavaScript 来渲染数学表达式。

## 核心架构

### 主要入口

| 文件 | 说明 |
|------|------|
| `src/lib.rs` | WASM 绑定入口点 |
| `src/katex.rs` | 主入口，包含三个公开函数 |

### 核心模块

| 模块路径 | 说明 |
|---------|------|
| `src/parse/` | LaTeX 解析逻辑 |
| `src/build/` | 解析树到 DOM 树/标记的转换 |
| `src/dom_tree/` | 虚拟 DOM 实现 |
| `src/mathML_tree/` | MathML 树生成 |
| `src/symbols/` | 符号定义 |
| `src/settings/` | 配置系统 |
| `src/metrics/` | 字体度量数据 |

### 公开 API

1. `render(expression, base_node, options)` - 直接渲染到 DOM 节点
2. `render_to_string_for_js(expression, settings)` - 生成 HTML 字符串（JS 可访问）
3. `render_to_string(expression, settings)` - 纯 Rust 字符串渲染

## 开发命令

### 构建

```bash
# 构建 WASM
wasm-pack build

# 构建 release 版本（优化体积）
wasm-pack build --release
```

### 测试

```bash
# 运行 Rust 单元测试
cargo test

# 运行 WASM 浏览器测试
wasm-pack test --headless --firefox

# 运行 render parity 测试
cargo test render_parity

# 使用过滤器运行特定测试
PARITY_CASE_FILTER="frac" cargo test parity
```

### Demo 开发

```bash
wasm-pack build
cd demo
npm install
npm start
```

## Diff Harness 测试工具

diff_harness 是用于对比 JS KaTeX 和 Rust WASM 实现差异的测试工具。

### 使用方法

```bash
wasm-pack build && node --experimental-wasm-modules tests/diff_harness.mjs <formulas.txt> [start_line] [end_line]
```

### 参数说明

- `formulas.txt`: 包含 LaTeX 公式的文件路径（每行一个公式）
- `start_line`: 起始行号（从 1 开始，可选，默认为 1）
- `end_line`: 结束行号（可选，默认为文件最后一行）

### 示例

```bash
# 测试整个文件
wasm-pack build && node --experimental-wasm-modules tests/diff_harness.mjs tests/fixtures/formulas.txt

# 测试指定行范围
wasm-pack build && node --experimental-wasm-modules tests/diff_harness.mjs tests/fixtures/formulas.txt 1 5
```

### 输出内容

对于每个公式，工具会输出：

- **JS parseTree**: JavaScript 版本的解析树结果
- **Rust parseTree**: Rust 版本的解析树结果
- **JS HTML**: JavaScript 版本的 HTML 渲染结果
- **Rust HTML**: Rust 版本的 HTML 渲染结果
- 每个步骤的执行耗时

### 核心文件

| 文件 | 说明 |
|------|------|
| `diff_harness/src/bin/diff_harness_cli.rs` | CLI 入口 |
| `diff_harness/src/harness.rs` | 核心 harness 逻辑 |
| `diff_harness/src/diff.rs` | 差异比较工具 |
| `diff_harness/src/minimize.rs` | 最小化测试用例 |
| `diff_harness/src/debug_parser.rs` | 调试解析器 |
| `diff_harness/scripts/js_parse_tree.js` | JS 解析树脚本 |
| `diff_harness/scripts/js_render_html.js` | JS HTML 渲染脚本 |

## 重要配置选项

- `displayMode`: 是否以 display 模式渲染
- `output`: 输出类型（html, mathml, htmlAndMathml）
- `throwOnError`: 是否对无效 LaTeX 抛出错误
- `errorColor`: 错误高亮颜色
- `maxSize`: 输出大小安全限制
- `maxExpand`: 宏展开次数限制
- `trust`: 是否允许潜在危险构造

## 相关文档

- `CLAUDE.md`: Claude Code 开发指南
- `structure.md`: JS KaTeX 到 Rust 的模块映射关系
- `README.md`: 项目简介
