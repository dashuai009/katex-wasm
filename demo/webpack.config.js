const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = (env, argv) => {
  const envArgs = env || {};
  console.log("env:", envArgs, "argv:", argv, "crateDirectory:", path.resolve(__dirname, ".."));
  const isProduction = argv.mode === "production";
  const publicPath = process.env.PUBLIC_PATH || "/";

  const wasmProfile = process.env.WASM_PROFILE || envArgs.wasmProfile || "";
  const wasmExtraArgs = process.env.WASM_PACK_EXTRA_ARGS || envArgs.wasmExtraArgs || "";
  const wasmArgs = process.env.WASM_PACK_ARGS || envArgs.wasmArgs || "--verbose";
  const wasmForceModeRaw =
    process.env.WASM_FORCE_MODE ||
    envArgs.wasmForceMode ||
    (isProduction ? "production" : "development");
  const wasmForceMode =
    wasmForceModeRaw === "production" || wasmForceModeRaw === "development"
      ? wasmForceModeRaw
      : undefined;

  const extraArgsParts = [];
  if (wasmProfile) {
    extraArgsParts.push(`--${wasmProfile}`);
  }
  if (wasmExtraArgs) {
    extraArgsParts.push(wasmExtraArgs);
  }
  const resolvedExtraArgs = extraArgsParts.join(" ").trim();

  let wasmPackPluginArgs = {
    crateDirectory: path.resolve(__dirname, ".."),
    args: wasmArgs,
    forceMode: "production",
    extraArgs: "--no-pack",
    pluginLogLevel: "info",
  }
  console.log("wasmPackPluginArgs:", wasmPackPluginArgs);

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
      new WasmPackPlugin(wasmPackPluginArgs),
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
