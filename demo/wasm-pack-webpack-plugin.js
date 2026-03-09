const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

function toArgsArray(args) {
  if (!args) return [];
  if (Array.isArray(args)) return args.filter(Boolean);
  return String(args)
    .trim()
    .split(/\s+/)
    .filter(Boolean);
}

function isExecutable(filePath) {
  try {
    fs.accessSync(filePath, fs.constants.X_OK);
    return true;
  } catch {
    return false;
  }
}

function findWasmPack() {
  if (process.env.WASM_PACK_PATH) {
    return process.env.WASM_PACK_PATH;
  }

  const exeNames = process.platform === 'win32' ? ['wasm-pack.exe', 'wasm-pack.cmd'] : ['wasm-pack'];
  const pathDirs = (process.env.PATH || '').split(path.delimiter).filter(Boolean);

  for (const dir of pathDirs) {
    for (const exeName of exeNames) {
      const fullPath = path.join(dir, exeName);
      if (isExecutable(fullPath)) {
        return fullPath;
      }
    }
  }

  const cargoPath = path.join(
    os.homedir(),
    '.cargo',
    'bin',
    process.platform === 'win32' ? 'wasm-pack.exe' : 'wasm-pack'
  );
  if (isExecutable(cargoPath)) {
    return cargoPath;
  }

  return null;
}

function run(bin, args, cwd) {
  return new Promise((resolve, reject) => {
    const child = spawn(bin, args, {
      cwd,
      stdio: 'inherit',
    });

    child.on('error', reject);
    child.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`wasm-pack exited with code ${code}`));
      }
    });
  });
}

class WasmPackWebpackPlugin {
  constructor(options = {}) {
    this.crateDirectory = options.crateDirectory;
    this.outDir = options.outDir || 'pkg';
    this.outName = options.outName || 'index';
    this.args = toArgsArray(options.args);

    if (!this.crateDirectory) {
      throw new Error('WasmPackWebpackPlugin requires `crateDirectory`.');
    }
  }

  async compile(mode) {
    const crateDirectory = path.resolve(this.crateDirectory);
    const stat = await fs.promises.stat(crateDirectory);

    if (!stat.isDirectory()) {
      throw new Error(`crateDirectory is not a directory: ${crateDirectory}`);
    }

    const wasmPackBin = findWasmPack();
    if (!wasmPackBin) {
      throw new Error(
        'Cannot find `wasm-pack`. Please install it first, or set WASM_PACK_PATH to the binary path.'
      );
    }

    const isProduction = mode === 'production';
    const wasmPackArgs = [
      'build',
      '--out-dir',
      this.outDir,
      '--out-name',
      this.outName,
      ...(isProduction ? ['--release'] : ['--dev']),
      ...this.args,
    ];

    console.log(`[WasmPackWebpackPlugin] ${wasmPackBin} ${wasmPackArgs.join(' ')}`);
    await run(wasmPackBin, wasmPackArgs, crateDirectory);
  }

  apply(compiler) {
    compiler.hooks.beforeCompile.tapPromise('WasmPackWebpackPlugin', async () => {
      const mode = compiler.options.mode || 'development';
      await this.compile(mode);
    });
  }
}

module.exports = WasmPackWebpackPlugin;
