import * as katex_wasm from "katex-wasm";
import katex from "katex";
import {
    Chart,
    LinearScale,
    PointElement,
    Legend,
    Title,
    Tooltip,
    ScatterController,
} from "chart.js";

Chart.register(LinearScale, PointElement, Legend, Title, Tooltip, ScatterController);

function parseBooleanParam(rawValue, defaultValue = false) {
    if (rawValue == null) return defaultValue;
    const normalized = String(rawValue).trim().toLowerCase();
    if (["1", "true", "yes", "on"].includes(normalized)) return true;
    if (["0", "false", "no", "off"].includes(normalized)) return false;
    return defaultValue;
}

function parseIntegerParam(rawValue, defaultValue, min = null) {
    if (rawValue == null || rawValue === "") return defaultValue;
    const value = Number.parseInt(rawValue, 10);
    if (!Number.isFinite(value)) return defaultValue;
    if (min != null && value < min) return min;
    return value;
}

/**
 * 从文本文件中读取指定行号范围的内容。
 */
async function loadFormulasFromFile(filePath, startLine = 1, endLine = null, maxFormulas = null) {
    const response = await fetch(filePath);
    if (!response.ok) {
        throw new Error(`Failed to load file: ${response.status} ${response.statusText}`);
    }

    const text = await response.text();
    const allLines = text.split("\n").filter((line) => line.trim() !== "");

    const totalLines = allLines.length;
    const actualStart = Math.max(1, startLine);
    const actualEnd = endLine !== null ? Math.min(endLine, totalLines) : totalLines;

    if (actualStart > totalLines || actualStart > actualEnd) {
        return [];
    }

    let selectedLines = allLines.slice(actualStart - 1, actualEnd);
    if (maxFormulas != null) {
        selectedLines = selectedLines.slice(0, maxFormulas);
    }

    console.log(
        `Loaded ${selectedLines.length} formulas from ${filePath} (lines ${actualStart}-${actualEnd})`
    );

    return selectedLines;
}

const urlParams = new URLSearchParams(window.location.search);
const autorun = parseBooleanParam(urlParams.get("autorun"), false);

const rawMode = (urlParams.get("mode") || "both").toLowerCase();
const mode = rawMode === "compute" ? "compute" : "both";

const CONFIG = {
    filePath: urlParams.get("file") || "./public/formulas.txt",
    startLine: parseIntegerParam(urlParams.get("start"), 1, 1),
    endLine: urlParams.get("end") ? parseIntegerParam(urlParams.get("end"), null, 1) : null,
    maxFormulas: urlParams.get("max") ? parseIntegerParam(urlParams.get("max"), null, 1) : null,
    mode,
    repeat: parseIntegerParam(urlParams.get("repeat"), 1, 1),
    warmupCount: parseIntegerParam(urlParams.get("warmup"), 1, 0),
    autorun,
    noChart: parseBooleanParam(urlParams.get("noChart"), autorun),
    noSummary: parseBooleanParam(urlParams.get("noSummary"), autorun),
};

window.__PERF_DONE__ = false;
window.__PERF_RESULT__ = null;

function createErrorNode(message) {
    const node = document.createElement("div");
    node.textContent = message;
    node.style.color = "red";
    return node;
}

function clearContainers(mathWasmContainer, mathKatexContainer, runIndex, totalRuns) {
    const runTag = totalRuns > 1 ? ` (run ${runIndex + 1}/${totalRuns})` : "";
    mathWasmContainer.innerHTML = `wasm${runTag}`;
    mathKatexContainer.innerHTML = `katex${runTag}`;
}

function summarizeErrors(perfData) {
    let wasmErrorCount = 0;
    let katexErrorCount = 0;
    for (const item of perfData) {
        if (item.wasmError) wasmErrorCount += 1;
        if (item.katexError) katexErrorCount += 1;
    }
    return { wasmErrorCount, katexErrorCount };
}

function computeStats(values) {
    const cleanValues = values.filter((v) => Number.isFinite(v));
    const count = cleanValues.length;
    if (count === 0) {
        return {
            count: 0,
            mean: 0,
            variance: 0,
            stddev: 0,
            min: 0,
            max: 0,
            median: 0,
            p95: 0,
            total: 0,
        };
    }

    const sorted = [...cleanValues].sort((a, b) => a - b);
    const total = cleanValues.reduce((sum, v) => sum + v, 0);
    const mean = total / count;
    const variance = cleanValues.reduce((sum, v) => sum + (v - mean) ** 2, 0) / count;
    const stddev = Math.sqrt(variance);
    const min = sorted[0];
    const max = sorted[count - 1];
    const median =
        count % 2 === 0
            ? (sorted[count / 2 - 1] + sorted[count / 2]) / 2
            : sorted[Math.floor(count / 2)];
    const p95Index = Math.ceil(count * 0.95) - 1;
    const p95 = sorted[Math.min(p95Index, count - 1)];

    return { count, mean, variance, stddev, min, max, median, p95, total };
}

function summarizePerfData(perfData) {
    const wasmTimes = perfData.map((d) => d.wasmTime);
    const katexTimes = perfData.map((d) => d.katexTime);
    const wasm = computeStats(wasmTimes);
    const katexStats = computeStats(katexTimes);
    const errors = summarizeErrors(perfData);

    return {
        sampleCount: perfData.length,
        wasm,
        katex: katexStats,
        ratio: {
            meanJsDivWasm: wasm.mean > 0 ? katexStats.mean / wasm.mean : 0,
            p95JsDivWasm: wasm.p95 > 0 ? katexStats.p95 / wasm.p95 : 0,
            totalJsDivWasm: wasm.total > 0 ? katexStats.total / wasm.total : 0,
        },
        errors,
    };
}

function formatMs(value) {
    return `${value.toFixed(3)} ms`;
}

async function runSingleBenchmark(formulas, settings, mathWasmContainer, mathKatexContainer) {
    const perfData = [];

    for (const formula of formulas) {
        const entry = {
            formula,
            length: formula.length,
            wasmTime: Number.NaN,
            katexTime: Number.NaN,
            wasmError: null,
            katexError: null,
        };

        // Measure Rust/WASM
        let startTime = performance.now();
        try {
            const wasmHtml = katex_wasm.renderToString(formula, settings);
            entry.wasmTime = performance.now() - startTime;

            if (CONFIG.mode === "both") {
                const node = document.createElement("span");
                node.innerHTML = wasmHtml;
                mathWasmContainer.append(node);
            }
        } catch (error) {
            entry.wasmError = error?.message || String(error);
            if (CONFIG.mode === "both") {
                mathWasmContainer.append(createErrorNode(`Error rendering formula: ${formula}`));
            }
        }

        // Measure JS KaTeX
        startTime = performance.now();
        try {
            if (CONFIG.mode === "compute") {
                katex.renderToString(formula, settings);
            } else {
                const node = document.createElement("div");
                katex.render(formula, node, settings);
                mathKatexContainer.append(node);
            }
            entry.katexTime = performance.now() - startTime;
        } catch (error) {
            entry.katexError = error?.message || String(error);
            if (CONFIG.mode === "both") {
                mathKatexContainer.append(createErrorNode(`Error rendering formula: ${formula}`));
            }
        }

        perfData.push(entry);
    }

    return perfData;
}

function createScatterPlot(perfData) {
    const filtered = perfData.filter(
        (item) => Number.isFinite(item.wasmTime) && Number.isFinite(item.katexTime)
    );

    if (filtered.length === 0) {
        return;
    }

    const chartContainer = document.createElement("div");
    chartContainer.style.marginTop = "50px";
    chartContainer.style.width = "800px";
    chartContainer.style.height = "600px";
    document.body.appendChild(chartContainer);

    const canvas = document.createElement("canvas");
    chartContainer.appendChild(canvas);

    const wasmData = filtered.map((d) => ({ x: d.length, y: d.wasmTime }));
    const katexData = filtered.map((d) => ({ x: d.length, y: d.katexTime }));

    new Chart(canvas, {
        type: "scatter",
        data: {
            datasets: [
                {
                    label: "Rust/WASM",
                    data: wasmData,
                    backgroundColor: "rgba(34, 139, 34, 0.7)",
                    borderColor: "rgba(34, 139, 34, 1)",
                    pointRadius: 6,
                    pointHoverRadius: 8,
                },
                {
                    label: "JavaScript",
                    data: katexData,
                    backgroundColor: "rgba(220, 20, 60, 0.7)",
                    borderColor: "rgba(220, 20, 60, 1)",
                    pointRadius: 6,
                    pointHoverRadius: 8,
                },
            ],
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                title: {
                    display: true,
                    text: "KaTeX Rendering Performance Comparison",
                    font: {
                        size: 18,
                        weight: "bold",
                    },
                },
                legend: {
                    display: true,
                    position: "top",
                },
                tooltip: {
                    callbacks: {
                        label(context) {
                            const formula = filtered[context.dataIndex].formula;
                            return [
                                `${context.dataset.label}:`,
                                `  Formula Length: ${context.parsed.x}`,
                                `  Time: ${context.parsed.y.toFixed(2)}ms`,
                                `  Formula: ${formula.substring(0, 30)}${formula.length > 30 ? "..." : ""}`,
                            ];
                        },
                    },
                },
            },
            scales: {
                x: {
                    type: "linear",
                    position: "bottom",
                    title: {
                        display: true,
                        text: "Formula Length (characters)",
                        font: {
                            size: 14,
                            weight: "bold",
                        },
                    },
                    grid: {
                        display: true,
                        color: "rgba(0, 0, 0, 0.1)",
                    },
                },
                y: {
                    type: "linear",
                    title: {
                        display: true,
                        text: "Rendering Time (ms)",
                        font: {
                            size: 14,
                            weight: "bold",
                        },
                    },
                    grid: {
                        display: true,
                        color: "rgba(0, 0, 0, 0.1)",
                    },
                    beginAtZero: true,
                },
            },
        },
    });
}

function createStatsSummary(perfData) {
    const summary = summarizePerfData(perfData);

    const rows = [
        { label: "Sample Count", wasm: summary.wasm.count, katex: summary.katex.count, isCount: true },
        { label: "Total Time", wasm: summary.wasm.total, katex: summary.katex.total },
        { label: "Mean (Average)", wasm: summary.wasm.mean, katex: summary.katex.mean },
        { label: "Median", wasm: summary.wasm.median, katex: summary.katex.median },
        { label: "Std Deviation", wasm: summary.wasm.stddev, katex: summary.katex.stddev },
        { label: "Variance", wasm: summary.wasm.variance, katex: summary.katex.variance },
        { label: "Min", wasm: summary.wasm.min, katex: summary.katex.min },
        { label: "Max", wasm: summary.wasm.max, katex: summary.katex.max },
        { label: "P95", wasm: summary.wasm.p95, katex: summary.katex.p95 },
    ];

    const container = document.createElement("div");
    container.style.marginTop = "30px";
    container.style.padding = "20px";
    container.style.fontFamily = "monospace";
    container.style.fontSize = "14px";
    container.style.maxWidth = "800px";

    let html = `
        <h3 style="margin-bottom: 12px; font-family: sans-serif;">Performance Statistics</h3>
        <table style="border-collapse: collapse; width: 100%;">
            <thead>
                <tr style="background: #f0f0f0;">
                    <th style="text-align: left; padding: 8px; border: 1px solid #ddd;">Metric</th>
                    <th style="text-align: right; padding: 8px; border: 1px solid #ddd; color: forestgreen;">Rust/WASM</th>
                    <th style="text-align: right; padding: 8px; border: 1px solid #ddd; color: crimson;">JavaScript</th>
                    <th style="text-align: right; padding: 8px; border: 1px solid #ddd;">Ratio (JS/WASM)</th>
                </tr>
            </thead>
            <tbody>
    `;

    for (const row of rows) {
        const wasmVal = row.isCount ? row.wasm : formatMs(row.wasm);
        const katexVal = row.isCount ? row.katex : formatMs(row.katex);
        const ratio = row.isCount ? "-" : row.wasm > 0 ? `${(row.katex / row.wasm).toFixed(2)}x` : "-";
        const ratioColor = !row.isCount && row.wasm > 0 ? (row.katex / row.wasm > 1 ? "forestgreen" : "crimson") : "inherit";

        html += `
            <tr>
                <td style="padding: 6px 8px; border: 1px solid #ddd; font-weight: bold;">${row.label}</td>
                <td style="padding: 6px 8px; border: 1px solid #ddd; text-align: right;">${wasmVal}</td>
                <td style="padding: 6px 8px; border: 1px solid #ddd; text-align: right;">${katexVal}</td>
                <td style="padding: 6px 8px; border: 1px solid #ddd; text-align: right; color: ${ratioColor};">${ratio}</td>
            </tr>
        `;
    }

    html += `</tbody></table>`;
    container.innerHTML = html;
    document.body.appendChild(container);
}

async function main() {
    console.log("CONFIG:", CONFIG);

    const mathWasmContainer = document.getElementById("math-wasm");
    const mathKatexContainer = document.getElementById("math-katex");

    const renderSettings = {
        displayMode: true,
        throwOnError: false,
        trust: true,
        maxSize: 200000,
        maxExpand: 1000,
    };

    let formulas = [];
    try {
        formulas = await loadFormulasFromFile(
            CONFIG.filePath,
            CONFIG.startLine,
            CONFIG.endLine,
            CONFIG.maxFormulas
        );
    } catch (error) {
        console.error(`Failed to load formulas from ${CONFIG.filePath}:`, error);
    }

    if (formulas.length === 0) {
        formulas = ["E=mc^2", "x+y=z", "a^2+b^2=c^2", "\\frac{1}{2}", "\\sqrt{x}"];
    }

    const allPerfData = [];
    const runSummaries = [];

    for (let runIndex = 0; runIndex < CONFIG.repeat; runIndex += 1) {
        if (CONFIG.mode === "both") {
            clearContainers(mathWasmContainer, mathKatexContainer, runIndex, CONFIG.repeat);
        }

        const runPerfData = await runSingleBenchmark(
            formulas,
            renderSettings,
            mathWasmContainer,
            mathKatexContainer
        );

        const trimmedRunData = runPerfData.slice(CONFIG.warmupCount);
        allPerfData.push(...trimmedRunData.map((entry) => ({ ...entry, runIndex: runIndex + 1 })));

        runSummaries.push({
            runIndex: runIndex + 1,
            ...summarizePerfData(trimmedRunData),
        });
    }

    if (!CONFIG.noChart && CONFIG.mode === "both") {
        createScatterPlot(allPerfData);
    }
    if (!CONFIG.noSummary) {
        createStatsSummary(allPerfData);
    }

    const aggregateSummary = summarizePerfData(allPerfData);
    const finalResult = {
        timestamp: new Date().toISOString(),
        config: CONFIG,
        environment: {
            userAgent: navigator.userAgent,
            hardwareConcurrency: navigator.hardwareConcurrency || null,
            deviceMemory: navigator.deviceMemory || null,
        },
        runSummaries,
        aggregateSummary,
    };

    window.__PERF_RESULT__ = finalResult;
    window.__PERF_DONE__ = true;

    console.log("PERF_RESULT_JSON:" + JSON.stringify(finalResult));
}

main().catch((error) => {
    console.error("Benchmark execution failed:", error);
    window.__PERF_RESULT__ = {
        timestamp: new Date().toISOString(),
        error: error?.message || String(error),
        config: CONFIG,
    };
    window.__PERF_DONE__ = true;
});
