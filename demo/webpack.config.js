const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyWebpackPlugin([
      { from: 'index.html', to: '.' },
      { from: '../pkg/katex_wasm_bg.wasm', to: '[name][ext]' },
      { from: 'formulas.txt', to: '.' }
    ])
  ],
};