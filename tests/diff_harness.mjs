#!/usr/bin/env node
/**
 * JS-based diff harness for comparing JS KaTeX and Rust WASM KaTeX output.
 *
 * Usage:
 *   node --experimental-wasm-modules diff_harness/scripts/diff_harness.mjs <formulas.txt> [start_line] [end_line] [--summary-only]
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

const args = process.argv.slice(2);

const summaryOnlyIndex = args.indexOf('--summary-only');
const summaryOnly = summaryOnlyIndex !== -1;
if (summaryOnly) {
    args.splice(summaryOnlyIndex, 1);
}

if (args.length < 1) {
    console.error('Usage: diff_harness.mjs <formulas.txt> [start_line] [end_line] [--summary-only]');
    console.error();
    console.error('  formulas.txt   Path to a file with one LaTeX formula per line');
    console.error('  start_line     Start line number (1-based, inclusive). Default: 1');
    console.error('  end_line       End line number (1-based, inclusive). Default: last line');
    console.error('  --summary-only Only output the final summary, not detailed results');
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

// ── Main loop ─────────────────────────────────────────────────────────────────

if (!summaryOnly) {
    console.error(`Processing lines ${startLine}-${endLine} from '${formulaFilePath}'`);
    console.error(`Project root: ${projectRoot}`);
    console.error();
}

let passCount = 0;
let failCount = 0;
let errorCount = 0;

for (let lineNum = startLine; lineNum <= endLine; lineNum++) {
    const formula = (allLines[lineNum - 1] || '').trim();
    if (!formula || formula.startsWith('#')) continue;

    if (!summaryOnly) {
        console.log(`${BOLD}========== Line ${lineNum} ==========${RESET}`);
        console.log(`Formula: ${formula}`);
        console.log();
    }

    // ── Print settings (once per formula) ─────────────────────────────────
    let rustSettings;
    try {
        // JS KaTeX processes options through SETTINGS_SCHEMA (defaults + processors)
        const jsSettings = new katex.Settings(renderSettings);
        const jsSettingsObj = {};
        for (const key of Object.keys(katex.SETTINGS_SCHEMA)) {
            jsSettingsObj[key] = jsSettings[key];
        }

        rustSettings = new katexWasm.Settings(renderSettings);
        const rustSettingsObj = rustSettings.toJsValue();

        if (!summaryOnly) {
            console.log(`${DIM}--- JS  Settings (after processing) ---${RESET}`);
            console.log(JSON.stringify(jsSettingsObj, null, 2));
            console.log(`${DIM}--- Rust Settings (after processing) ---${RESET}`);
            console.log(JSON.stringify(rustSettingsObj, null, 2));
            console.log();
        }
    } catch (error) {
        if (!summaryOnly) {
            console.log(`${YELLOW}Warning: failed to print settings: ${error.message}${RESET}`);
        }
    }

    // ── JS KaTeX HTML ─────────────────────────────────────────────────────
    let jsHtml = '';
    let jsTime = 0;
    try {
        const jsStart = performance.now();
        jsHtml = katex.renderToString(formula, renderSettings);
        jsTime = performance.now() - jsStart;
    } catch (error) {
        jsHtml = `JS_ERROR: ${error.message || error}`;
        errorCount++;
    }

    // ── Rust WASM HTML ────────────────────────────────────────────────────
    let rustHtml = '';
    let rustTime = 0;
    try {
        const rustStart = performance.now();
        rustHtml = katexWasm.renderToString(formula, renderSettings);
        rustTime = performance.now() - rustStart;
    } catch (error) {
        rustHtml = `RUST_ERROR: ${error.message || error}`;
        errorCount++;
    }

    // ── Output ────────────────────────────────────────────────────────────
    if (!summaryOnly) {
        console.log(`${DIM}--- JS HTML (${jsTime.toFixed(2)}ms) ---${RESET}`);
        console.log(jsHtml);
        console.log();
        console.log(`${DIM}--- Rust HTML (${rustTime.toFixed(2)}ms) ---${RESET}`);
        console.log(rustHtml);
        console.log();
    }

    // ── Compare ───────────────────────────────────────────────────────────
    if (jsHtml === rustHtml) {
        if (!summaryOnly) {
            console.log(`${GREEN}✓ MATCH${RESET}`);
        }
        passCount++;
    } else {
        if (!summaryOnly) {
            console.log(`${RED}✗ MISMATCH${RESET}`);
            failCount++;

            const diff = highlightDiff(jsHtml, rustHtml);
            if (diff) {
                console.log(`  First difference at position ${diff.position}:`);
                console.log(`  ${CYAN}JS  :${RESET} ...${diff.jsSnippet}...`);
                console.log(`  ${YELLOW}Rust:${RESET} ...${diff.rustSnippet}...`);
                console.log(`        ${RED}${diff.marker}${RESET}`);
            }
        } else {
            failCount++;
        }
    }
    if (!summaryOnly) {
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
