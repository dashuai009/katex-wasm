const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env, argv) => {
  const envArgs = env || {};
  const isProduction = argv.mode === 'production';
  const publicPath = process.env.PUBLIC_PATH || '/';

  const wasmPluginOptions = {
    crateDirectory: path.resolve(__dirname, '..'),
    outDir: 'pkg',
  };

  if (process.env.WASM_PROFILE === 'profiling') {
    wasmPluginOptions.extraArgs = '--profiling';
    wasmPluginOptions.forceMode = 'is_not_development';
  }

  console.log('webpack mode:', isProduction ? 'production' : 'development');
  console.log('wasm plugin options:', wasmPluginOptions);

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
        template: 'index.html',
      }),
      new WasmPackPlugin(wasmPluginOptions),
      new CopyPlugin({
        patterns: [{ from: 'public', to: 'public' }],
      }),
    ],
    mode: isProduction ? 'production' : 'development',
    experiments: {
      asyncWebAssembly: true,
    },
  };
};
