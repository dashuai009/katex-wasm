const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = (env, argv) => {
  const isProduction = argv.mode === "production";
  const publicPath = process.env.PUBLIC_PATH || "/";

  return {
    entry: './bootstrap.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'index.js',
      publicPath,
      clean: true,
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      ...([
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "..")
        }),
      ]),
      new CopyPlugin({
        patterns: [
          { from: "public", to: "public" }
        ],
      }),
    ],
    mode: isProduction ? 'production' : 'development',
    experiments: {
      asyncWebAssembly: true
    }
  };
};
