const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackWebpackPlugin = require('./wasm-pack-webpack-plugin');

module.exports = (env, argv) => {
  const envArgs = env || {};
  const isProduction = argv.mode === 'production';
  const publicPath = process.env.PUBLIC_PATH || '/';

  const wasmArgsRaw =
    process.env.WASM_PACK_ARGS ||
    envArgs.wasmArgs ||
    '--no-pack';

  const wasmPluginOptions = {
    crateDirectory: path.resolve(__dirname, '..'),
    outDir: 'pkg',
    outName: 'index',
    args: wasmArgsRaw,
  };

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
      new WasmPackWebpackPlugin(wasmPluginOptions),
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
