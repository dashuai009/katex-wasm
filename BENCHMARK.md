# Node.js 调用 WASM 性能分析

本文档给出可复现的基准与 profile 流程，目标是把总耗时拆分为：

1. JS 侧开销
2. JS↔WASM 边界成本（调用/拷贝/编解码）
3. WASM 内部计算

## 1. 环境准备

```bash
wasm-pack build --release --target nodejs
node -v
```

建议记录：CPU、内存、OS、Node 版本、构建模式。

## 2. 主基准（p50/p95/p99 + 吞吐 + 内存）

```bash
node --expose-gc bench/bench.js
```

可选参数：

```bash
WARMUP_ROUNDS=8 MEASURE_ROUNDS=20 INNER_LOOPS=50 FORCE_GC=1 node --expose-gc bench/bench.js
```

输出：

- small / medium / large 三档输入
- mean / p50 / p95 / p99
- QPS
- RSS / heapUsed
- 最近一次结果写入 `profiles/bench-last.json`

## 3. CPU Profile（宏观归因）

```bash
node --expose-gc --cpu-prof --cpu-prof-interval=100 bench/bench.js
```

Node 会在当前目录生成 `CPU.*.cpuprofile`。建议移动到 `profiles/`：

```bash
mv CPU.*.cpuprofile profiles/baseline.cpuprofile
```

## 4. 微基准：边界成本拆分

### 4.1 空调用成本（跨边界）

```bash
node bench/micro_noop.js
```

可调：`CALLS`、`WARMUP`。

### 4.2 拷贝成本（JS→WASM / WASM→JS）

```bash
node bench/micro_copy.js
```

可调：`BYTES`、`ROUNDS`。

### 4.3 字符串编解码成本

```bash
node bench/micro_codec.js
```

可调：`ROUNDS`、`PAYLOAD`。

## 5. 对比表模板（优化前后）

| 指标 | Baseline | Optimized | 变化 |
|---|---:|---:|---:|
| small mean (ms) |  |  |  |
| small p95 (ms) |  |  |  |
| medium mean (ms) |  |  |  |
| medium p95 (ms) |  |  |  |
| large mean (ms) |  |  |  |
| large p95 (ms) |  |  |  |
| QPS (large) |  |  |  |
| RSS (MB) |  |  |  |
| heapUsed (MB) |  |  |  |
| noop ns/call |  |  |  |
| copy JS→WASM (GB/s) |  |  |  |
| copy WASM→JS (GB/s) |  |  |  |
| codec ns/round |  |  |  |

## 6. 回归建议

- CI 中运行轻量参数：`MEASURE_ROUNDS=5 INNER_LOOPS=20 node bench/bench.js`
- 保留代表性 `profiles/*.cpuprofile`（baseline + optimized）
- 若瓶颈在 WASM 内部，再追加 `perf` 采样分析
