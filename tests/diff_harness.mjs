#!/usr/bin/env node
/**
 * JS-based diff harness for comparing JS KaTeX and Rust WASM KaTeX output.
 *
 * Usage:
 *   node --experimental-wasm-modules diff_harness/scripts/diff_harness.mjs <formulas.txt> [start_line] [end_line] [--log-level <summary|normal|debug>]
 *
 * Example:
 *   node --experimental-wasm-modules diff_harness/scripts/diff_harness.mjs tests/fixtures/formulas.txt 1 5
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { performance } from 'perf_hooks';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.resolve(__dirname, '..');

// ── Argument parsing ──────────────────────────────────────────────────────────

const rawArgs = process.argv.slice(2);
const args = [];
let logLevel;

for (let i = 0; i < rawArgs.length; i++) {
    const arg = rawArgs[i];

    if (arg === '--log-level') {
        const nextArg = rawArgs[i + 1];
        if (!nextArg) {
            console.error('Missing value for --log-level');
            process.exit(1);
        }
        logLevel = nextArg;
        i++;
        continue;
    }

    if (arg.startsWith('--log-level=')) {
        logLevel = arg.slice('--log-level='.length);
        continue;
    }

    args.push(arg);
}

const validLogLevels = new Set(['summary', 'error', 'normal', 'debug']);
logLevel ??= 'debug';

if (!validLogLevels.has(logLevel)) {
    console.error(`Invalid log level: '${logLevel}'. Expected one of: summary, normal, debug`);
    process.exit(1);
}

const showDetails = logLevel !== 'summary';
const errorOnly = logLevel === 'error';
const showAllFormulaLogs = logLevel === 'normal' || logLevel === 'debug';
const showSettings = logLevel === 'debug';

if (args.length < 1) {
    console.error('Usage: diff_harness.mjs <formulas.txt> [start_line] [end_line] [--log-level <summary|error|normal|debug>]');
    console.error();
    console.error('  formulas.txt   Path to a file with one LaTeX formula per line');
    console.error('  start_line     Start line number (1-based, inclusive). Default: 1');
    console.error('  end_line       End line number (1-based, inclusive). Default: last line');
    console.error('  --log-level    summary: only print the final summary');
    console.error('                 error: only print formulas with render errors or mismatches');
    console.error('                 normal: print per-formula render results and match status');
    console.error('                 debug: normal output plus processed JS/Rust settings');
    console.error('                 default: debug');
    console.error();
    console.error('Example:');
    console.error('  node --experimental-wasm-modules diff_harness/scripts/diff_harness.mjs tests/fixtures/formulas.txt 1 5');
    process.exit(1);
}

const formulaFilePath = path.resolve(args[0]);
const content = fs.readFileSync(formulaFilePath, 'utf8');
const allLines = content.split('\n');
const totalLines = allLines.length;

const startLine = Math.max(1, parseInt(args[1]) || 1);
const endLine = Math.min(totalLines, parseInt(args[2]) || totalLines);

if (startLine > endLine || startLine > totalLines) {
    console.error(`Invalid line range: ${startLine}-${endLine} (file has ${totalLines} lines)`);
    process.exit(1);
}

// ── Load JS KaTeX ─────────────────────────────────────────────────────────────

const katexPath = path.join(projectRoot, 'KaTeX/dist/katex.mjs');
const katex = (await import(katexPath)).default;

// ── Load WASM KaTeX ───────────────────────────────────────────────────────────

const wasmPkgPath = path.join(projectRoot, 'pkg/katex_wasm.js');
const katexWasm = await import(wasmPkgPath);

// ── Settings ──────────────────────────────────────────────────────────────────

const renderSettings = {
    displayMode: true,
    output: 'html',
    throwOnError: false,
    strict: 'ignore',
    trust: true,
    maxSize: 200000,
    maxExpand: 1000,
};

// ── Color helpers ─────────────────────────────────────────────────────────────

const RED = '\x1b[31m';
const GREEN = '\x1b[32m';
const YELLOW = '\x1b[33m';
const CYAN = '\x1b[36m';
const RESET = '\x1b[0m';
const BOLD = '\x1b[1m';
const DIM = '\x1b[2m';

// ── Diff helper ───────────────────────────────────────────────────────────────

function findFirstDiffIndex(strA, strB) {
    const minLen = Math.min(strA.length, strB.length);
    for (let i = 0; i < minLen; i++) {
        if (strA[i] !== strB[i]) return i;
    }
    if (strA.length !== strB.length) return minLen;
    return -1;
}

function highlightDiff(strA, strB) {
    const diffIdx = findFirstDiffIndex(strA, strB);
    if (diffIdx === -1) return null;

    const contextStart = Math.max(0, diffIdx - 40);
    const contextEnd = diffIdx + 60;

    const snippetA = strA.substring(contextStart, contextEnd);
    const snippetB = strB.substring(contextStart, contextEnd);
    const markerPos = diffIdx - contextStart;

    return {
        position: diffIdx,
        jsSnippet: snippetA,
        rustSnippet: snippetB,
        marker: ' '.repeat(markerPos) + '^',
    };
}

function htmlMatchesWithinTolerance(jsHtml, rustHtml) {
    const jsMatches = [...jsHtml.matchAll(/(-?\d+(?:\.\d+)?)em\b/g)];
    const rustMatches = [...rustHtml.matchAll(/(-?\d+(?:\.\d+)?)em\b/g)];
    let jsIndex = 0;
    let rustIndex = 0;

    if (jsMatches.length !== rustMatches.length) {
        return false;
    }

    for (let i = 0; i < jsMatches.length; i++) {
        const jsMatch = jsMatches[i];
        const rustMatch = rustMatches[i];
        const jsStatic = jsHtml.slice(jsIndex, jsMatch.index);
        const rustStatic = rustHtml.slice(rustIndex, rustMatch.index);
        if (jsStatic !== rustStatic) {
            return false;
        }

        const jsValue = Number(jsMatch[1]);
        const rustValue = Number(rustMatch[1]);
        if (Math.abs(jsValue - rustValue) > 0.001) {
            return false;
        }

        jsIndex = jsMatch.index + jsMatch[0].length;
        rustIndex = rustMatch.index + rustMatch[0].length;
    }

    return jsHtml.slice(jsIndex) === rustHtml.slice(rustIndex);
}

// ── Main loop ─────────────────────────────────────────────────────────────────

if (showDetails) {
    console.error(`Processing lines ${startLine}-${endLine} from '${formulaFilePath}'`);
    console.error(`Project root: ${projectRoot}`);
    console.error(`Log level: ${logLevel}`);
    console.error();
}

let passCount = 0;
let failCount = 0;
let errorCount = 0;

for (let lineNum = startLine; lineNum <= endLine; lineNum++) {
    const formula = (allLines[lineNum - 1] || '').trim();
    if (!formula || formula.startsWith('#')) continue;

    // ── Print settings (once per formula) ─────────────────────────────────
    let jsSettingsObj = null;
    let rustSettingsObj = null;
    let settingsWarning = null;
    if (showSettings) {
        try {
            // JS KaTeX processes options through SETTINGS_SCHEMA (defaults + processors)
            const jsSettings = new katex.Settings(renderSettings);
            jsSettingsObj = {};
            for (const key of Object.keys(katex.SETTINGS_SCHEMA)) {
                jsSettingsObj[key] = jsSettings[key];
            }

            const rustSettings = new katexWasm.Settings(renderSettings);
            rustSettingsObj = rustSettings.toJsValue();
        } catch (error) {
            settingsWarning = error.message;
        }
    }

    // ── JS KaTeX HTML ─────────────────────────────────────────────────────
    let jsHtml = '';
    let jsTime = 0;
    let jsHadError = false;
    try {
        const jsStart = performance.now();
        jsHtml = katex.renderToString(formula, renderSettings);
        jsTime = performance.now() - jsStart;
    } catch (error) {
        jsHtml = `JS_ERROR: ${error.message || error}`;
        jsHadError = true;
        errorCount++;
    }

    // ── Rust WASM HTML ────────────────────────────────────────────────────
    let rustHtml = '';
    let rustTime = 0;
    let rustHadError = false;
    try {
        const rustStart = performance.now();
        rustHtml = katexWasm.renderToString(formula, renderSettings);
        rustTime = performance.now() - rustStart;
    } catch (error) {
        rustHtml = `RUST_ERROR: ${error.message || error}`;
        rustHadError = true;
        errorCount++;
    }

    // ── Compare ───────────────────────────────────────────────────────────
    const matched = jsHtml === rustHtml || htmlMatchesWithinTolerance(jsHtml, rustHtml);
    const shouldLogFormula = showAllFormulaLogs || (errorOnly && (jsHadError || rustHadError || !matched));

    if (shouldLogFormula) {
        console.log(`${BOLD}========== Line ${lineNum} ==========${RESET}`);
        console.log(`Formula: ${formula}`);
        console.log();

        if (showSettings) {
            if (settingsWarning) {
                console.log(`${YELLOW}Warning: failed to print settings: ${settingsWarning}${RESET}`);
            } else {
                console.log(`${DIM}--- JS  Settings (after processing) ---${RESET}`);
                console.log(JSON.stringify(jsSettingsObj, null, 2));
                console.log(`${DIM}--- Rust Settings (after processing) ---${RESET}`);
                console.log(JSON.stringify(rustSettingsObj, null, 2));
            }
            console.log();
        }

        console.log(`${DIM}--- JS HTML (${jsTime.toFixed(2)}ms) ---${RESET}`);
        console.log(jsHtml);
        console.log();
        console.log(`${DIM}--- Rust HTML (${rustTime.toFixed(2)}ms) ---${RESET}`);
        console.log(rustHtml);
        console.log();
    }

    if (matched) {
        if (shouldLogFormula) {
            console.log(`${GREEN}✓ MATCH${RESET}`);
        }
        passCount++;
    } else {
        if (shouldLogFormula) {
            console.log(`${RED}✗ MISMATCH${RESET}`);

            const diff = highlightDiff(jsHtml, rustHtml);
            if (diff) {
                console.log(`  First difference at position ${diff.position}:`);
                console.log(`  ${CYAN}JS  :${RESET} ...${diff.jsSnippet}...`);
                console.log(`  ${YELLOW}Rust:${RESET} ...${diff.rustSnippet}...`);
                console.log(`        ${RED}${diff.marker}${RESET}`);
            }
        }
        failCount++;
    }
    if (shouldLogFormula) {
        console.log();
    }
}

// ── Summary ───────────────────────────────────────────────────────────────────

console.log(`${BOLD}══════════ Summary ══════════${RESET}`);
console.log(`  ${GREEN}Pass:${RESET}   ${passCount}`);
console.log(`  ${RED}Fail:${RESET}   ${failCount}`);
if (errorCount > 0) {
    console.log(`  ${YELLOW}Errors:${RESET} ${errorCount}`);
}
console.log(`  Total:  ${passCount + failCount}`);

process.exit(failCount > 0 ? 1 : 0);
