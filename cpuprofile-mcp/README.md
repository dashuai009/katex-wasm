# cpuprofile-mcp (Rust)

一个使用 Rust 实现的 MCP Server，用于分析 Chrome / V8 `.cpuprofile` 文件。

## 工具列表

- `summarize_cpuprofile`
- `inspect_cpuprofile_node`
- `summarize_cpuprofile_by_url`
- `get_cpuprofile_hot_paths`
- `get_cpuprofile_bottom_up`
- `diff_cpuprofiles`
- `diagnose_cpuprofile`

## 本地运行

```bash
cargo run -p cpuprofile-mcp
```

## MCP 客户端配置示例

```json
{
  "mcpServers": {
    "cpuprofile": {
      "command": "cargo",
      "args": [
        "run",
        "--quiet",
        "-p",
        "cpuprofile-mcp"
      ],
      "cwd": "/workspace/katex-wasm"
    }
  }
}
```
