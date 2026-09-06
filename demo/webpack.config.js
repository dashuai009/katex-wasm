const path = require("path");
const { DefinePlugin } = require("webpack");
const buildInfo = require("../scripts/build-state.cjs").assertFresh();
buildInfo.katexRsWasmSha256 = require("node:crypto")
  .createHash("sha256")
  .update(
    require("node:fs").readFileSync(
      path.resolve(__dirname, "../third-party/katex-rs-pkg/katex_bg.wasm"),
    ),
  )
  .digest("hex");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].[contenthash].js",
    publicPath: process.env.PUBLIC_PATH || "auto",
    clean: true,
  },
  resolve: {
    alias: {
      "katex-rs": path.resolve(
        __dirname,
        "../third-party/katex-rs-pkg/katex.js",
      ),
    },
  },
  experiments: { asyncWebAssembly: true },
  plugins: [
    new DefinePlugin({ __BUILD_INFO__: JSON.stringify(buildInfo) }),
    new HtmlWebpackPlugin({ template: "index.html" }),
    new CopyPlugin({
      patterns: [
        { from: "public", to: "public" },
        { from: "node_modules/katex/dist/katex.min.css", to: "katex.min.css" },
        { from: "node_modules/katex/dist/fonts", to: "fonts" },
      ],
    }),
  ],
  devServer: { host: "127.0.0.1" },
};
