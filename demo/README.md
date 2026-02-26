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
