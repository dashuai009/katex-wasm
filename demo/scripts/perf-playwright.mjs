#!/usr/bin/env node
import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { spawn } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const demoDir = path.resolve(__dirname, "..");

function parseArgs(argv) {
    const args = {};
    for (let i = 0; i < argv.length; i += 1) {
        const token = argv[i];
        if (!token.startsWith("--")) continue;

        const stripped = token.slice(2);
        const eqPos = stripped.indexOf("=");
        if (eqPos >= 0) {
            const key = stripped.slice(0, eqPos);
            const value = stripped.slice(eqPos + 1);
            args[key] = value;
            continue;
        }

        const key = stripped;
        const next = argv[i + 1];
        if (!next || next.startsWith("--")) {
            args[key] = "true";
        } else {
            args[key] = next;
            i += 1;
        }
    }
    return args;
}

function parseBool(value, fallback = false) {
    if (value == null) return fallback;
    const normalized = String(value).trim().toLowerCase();
    if (["1", "true", "yes", "on"].includes(normalized)) return true;
    if (["0", "false", "no", "off"].includes(normalized)) return false;
    return fallback;
}

function parseIntOrDefault(value, fallback) {
    if (value == null || value === "") return fallback;
    const num = Number.parseInt(value, 10);
    return Number.isFinite(num) ? num : fallback;
}

function resolveOptionalPathArg(rawValue, defaultRelativePath) {
    if (rawValue == null) return null;
    const normalized = String(rawValue).trim();
    if (normalized === "" || normalized === "true") {
        return path.resolve(process.cwd(), defaultRelativePath);
    }
    return path.resolve(process.cwd(), normalized);
}

async function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForServer(url, timeoutMs) {
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
        try {
            const response = await fetch(url, { method: "GET" });
            if (response.ok) return;
        } catch (_error) {
            // Ignore and retry.
        }
        await sleep(500);
    }
    throw new Error(`Timed out waiting for server: ${url}`);
}

async function isServerReady(url) {
    try {
        const response = await fetch(url, { method: "GET" });
        return response.ok;
    } catch (_error) {
        return false;
    }
}

function spawnDevServer(host, port) {
    const npmCmd = process.platform === "win32" ? "npm.cmd" : "npm";
    const child = spawn(npmCmd, ["run", "serve:profiling", "--", "--host", host, "--port", String(port)], {
        cwd: demoDir,
        stdio: ["ignore", "pipe", "pipe"],
        detached: process.platform !== "win32",
    });

    child.stdout.on("data", (chunk) => {
        process.stdout.write(`[serve] ${chunk}`);
    });
    child.stderr.on("data", (chunk) => {
        process.stderr.write(`[serve] ${chunk}`);
    });

    return child;
}

function stopServer(serverProcess) {
    if (!serverProcess || serverProcess.killed || serverProcess.pid == null) {
        return;
    }

    try {
        if (process.platform === "win32") {
            const taskkill = spawn("taskkill", ["/pid", String(serverProcess.pid), "/T", "/F"], {
                stdio: "ignore",
            });
            taskkill.unref();
        } else {
            process.kill(-serverProcess.pid, "SIGTERM");
        }
    } catch (_error) {
        try {
            serverProcess.kill("SIGTERM");
        } catch (__error) {
            // Ignore.
        }
    }
}

async function prepareInputFile(inputPath) {
    if (!inputPath) {
        return "./public/formulas.txt";
    }

    const resolvedInputPath = path.resolve(process.cwd(), inputPath);
    const inputExt = path.extname(resolvedInputPath).toLowerCase() || ".lst";
    const targetFilename = `perf-input${inputExt}`;
    const targetRelPath = `./public/${targetFilename}`;
    const targetAbsPath = path.resolve(demoDir, "public", targetFilename);

    await fs.copyFile(resolvedInputPath, targetAbsPath);
    console.log(
        `Copied perf input to public: ${path.relative(process.cwd(), resolvedInputPath)} -> ${targetRelPath}`
    );
    return targetRelPath;
}

async function loadPlaywright() {
    try {
        const module = await import("playwright");
        return module.chromium;
    } catch (error) {
        throw new Error(
            `Failed to load playwright. Run \"cd demo && npm install\" first. Original error: ${error.message}`
        );
    }
}

function printSummary(result) {
    const summary = result.aggregateSummary;
    if (!summary) {
        console.log("No aggregate summary found in result.");
        return;
    }

    console.log("\n=== Perf Summary ===");
    console.log(`sampleCount: ${summary.sampleCount}`);
    console.log(`wasm.mean: ${summary.wasm.mean.toFixed(4)} ms`);
    console.log(`wasm.p95:  ${summary.wasm.p95.toFixed(4)} ms`);
    console.log(`js.mean:   ${summary.katex.mean.toFixed(4)} ms`);
    console.log(`js.p95:    ${summary.katex.p95.toFixed(4)} ms`);
    console.log(`mean ratio (JS/WASM): ${summary.ratio.meanJsDivWasm.toFixed(4)}x`);
    console.log(`p95 ratio  (JS/WASM): ${summary.ratio.p95JsDivWasm.toFixed(4)}x`);
}

function evaluateThresholds(result, args) {
    const summary = result.aggregateSummary;
    if (!summary) return [];

    const violations = [];

    const maxWasmMean = args["max-wasm-mean-ms"] != null ? Number(args["max-wasm-mean-ms"]) : null;
    const maxWasmP95 = args["max-wasm-p95-ms"] != null ? Number(args["max-wasm-p95-ms"]) : null;

    if (Number.isFinite(maxWasmMean) && summary.wasm.mean > maxWasmMean) {
        violations.push(
            `wasm.mean ${summary.wasm.mean.toFixed(4)} ms exceeds threshold ${maxWasmMean.toFixed(4)} ms`
        );
    }

    if (Number.isFinite(maxWasmP95) && summary.wasm.p95 > maxWasmP95) {
        violations.push(
            `wasm.p95 ${summary.wasm.p95.toFixed(4)} ms exceeds threshold ${maxWasmP95.toFixed(4)} ms`
        );
    }

    return violations;
}

async function compareWithBaseline(result, baselinePath, maxRegressionPct) {
    const resolvedPath = path.resolve(process.cwd(), baselinePath);
    const baselineRaw = await fs.readFile(resolvedPath, "utf8");
    const baseline = JSON.parse(baselineRaw);

    const currentSummary = result.aggregateSummary;
    const baselineSummary = baseline.aggregateSummary;
    if (!currentSummary || !baselineSummary) {
        return ["Baseline file missing aggregateSummary"];
    }

    const violations = [];
    const checks = [
        { metric: "wasm.mean", current: currentSummary.wasm.mean, baseline: baselineSummary.wasm.mean },
        { metric: "wasm.p95", current: currentSummary.wasm.p95, baseline: baselineSummary.wasm.p95 },
        { metric: "wasm.total", current: currentSummary.wasm.total, baseline: baselineSummary.wasm.total },
    ];

    for (const item of checks) {
        if (!Number.isFinite(item.current) || !Number.isFinite(item.baseline) || item.baseline <= 0) {
            continue;
        }
        const regressionPct = ((item.current - item.baseline) / item.baseline) * 100;
        if (regressionPct > maxRegressionPct) {
            violations.push(
                `${item.metric} regressed by ${regressionPct.toFixed(2)}% (baseline=${item.baseline.toFixed(4)}, current=${item.current.toFixed(4)})`
            );
        }
    }

    return violations;
}

async function main() {
    const args = parseArgs(process.argv.slice(2));

    const host = args.host || "127.0.0.1";
    const port = parseIntOrDefault(args.port, 4173);
    const timeoutMs = parseIntOrDefault(args.timeout, 180000);
    const headless = parseBool(args.headless, true);

    const repeat = parseIntOrDefault(args.repeat, 3);
    const warmup = parseIntOrDefault(args.warmup, 1);
    const start = parseIntOrDefault(args.start, 1);
    const end = args.end != null ? parseIntOrDefault(args.end, null) : null;
    const max = args.max != null ? parseIntOrDefault(args.max, null) : null;
    const mode = (args.mode || "compute").toLowerCase() === "both" ? "both" : "compute";

    const outputPath = path.resolve(
        process.cwd(),
        args.output || path.join("demo", "perf-results", `perf-${Date.now()}.json`)
    );
    const cpuProfilePath = resolveOptionalPathArg(
        args["cpu-profile"],
        path.join("demo", "perf-results", `perf-${Date.now()}.cpuprofile`)
    );
    const cpuSamplingIntervalUs = parseIntOrDefault(args["cpu-sampling-interval-us"], 100);
    const cpuSlowdownRate = 4;

    const baseUrl = args.url || `http://${host}:${port}/`;
    const shouldStartServer = !args.url;

    const fileParam = await prepareInputFile(args.input || null);

    let serverProcess = null;
    let browser = null;
    let cdpSession = null;
    let cpuProfileStarted = false;

    try {
        if (shouldStartServer) {
            const alreadyRunning = await isServerReady(baseUrl);
            if (alreadyRunning) {
                console.log(`Reusing existing server: ${baseUrl}`);
            } else {
                serverProcess = spawnDevServer(host, port);
                await waitForServer(baseUrl, timeoutMs);
            }
        }

        const chromium = await loadPlaywright();
        browser = await chromium.launch({ headless });
        const page = await browser.newPage();
        cdpSession = await page.context().newCDPSession(page);
        await cdpSession.send("Emulation.setCPUThrottlingRate", {
            rate: cpuSlowdownRate,
        });
        console.log(`CPU throttling enabled: ${cpuSlowdownRate}x slowdown`);

        if (cpuProfilePath) {
            await cdpSession.send("Profiler.enable");
            if (cpuSamplingIntervalUs > 0) {
                await cdpSession.send("Profiler.setSamplingInterval", {
                    interval: cpuSamplingIntervalUs,
                });
            }
            await cdpSession.send("Profiler.start");
            cpuProfileStarted = true;
            console.log(`CPU profile recording enabled: ${cpuProfilePath}`);
        }

        page.on("console", (msg) => {
            if (msg.type() === "error") {
                console.error(`[page] ${msg.text()}`);
            }
        });

        const targetUrl = new URL(baseUrl);
        targetUrl.searchParams.set("autorun", "1");
        targetUrl.searchParams.set("mode", mode);
        targetUrl.searchParams.set("repeat", String(repeat));
        targetUrl.searchParams.set("warmup", String(warmup));
        targetUrl.searchParams.set("file", fileParam);
        targetUrl.searchParams.set("start", String(start));
        targetUrl.searchParams.set("noChart", "1");
        targetUrl.searchParams.set("noSummary", "1");

        if (end != null) {
            targetUrl.searchParams.set("end", String(end));
        }
        if (max != null) {
            targetUrl.searchParams.set("max", String(max));
        }

        console.log(`Opening benchmark URL: ${targetUrl.toString()}`);

        await page.goto(targetUrl.toString(), {
            waitUntil: "networkidle",
            timeout: timeoutMs,
        });

        await page.waitForFunction(() => window.__PERF_DONE__ === true, {
            timeout: timeoutMs,
        });

        const result = await page.evaluate(() => window.__PERF_RESULT__);
        if (!result) {
            throw new Error("Benchmark finished but no result found in window.__PERF_RESULT__");
        }
        if (result.error) {
            throw new Error(`Benchmark error: ${result.error}`);
        }

        await fs.mkdir(path.dirname(outputPath), { recursive: true });
        await fs.writeFile(outputPath, `${JSON.stringify(result, null, 2)}\n`, "utf8");

        console.log(`Result written to: ${outputPath}`);
        printSummary(result);

        let violations = evaluateThresholds(result, args);

        if (args.baseline) {
            const maxRegressionPct = Number.isFinite(Number(args["max-regression-pct"]))
                ? Number(args["max-regression-pct"])
                : 10;
            const baselineViolations = await compareWithBaseline(
                result,
                args.baseline,
                maxRegressionPct
            );
            violations = violations.concat(baselineViolations);
        }

        if (violations.length > 0) {
            console.error("\nThreshold check failed:");
            for (const violation of violations) {
                console.error(`- ${violation}`);
            }
            process.exitCode = 1;
            return;
        }

        console.log("\nBenchmark completed successfully.");
    } finally {
        if (cpuProfileStarted && cdpSession) {
            try {
                const cpuProfileResult = await cdpSession.send("Profiler.stop");
                await fs.mkdir(path.dirname(cpuProfilePath), { recursive: true });
                await fs.writeFile(
                    cpuProfilePath,
                    `${JSON.stringify(cpuProfileResult.profile, null, 2)}\n`,
                    "utf8"
                );
                console.log(`CPU profile written to: ${cpuProfilePath}`);
            } catch (error) {
                console.error(
                    `Failed to save CPU profile: ${error.message || String(error)}`
                );
                if (process.exitCode == null || process.exitCode === 0) {
                    process.exitCode = 1;
                }
            }
        }

        if (browser) {
            await browser.close();
        }

        stopServer(serverProcess);
    }
}

main().catch((error) => {
    console.error(error.stack || error.message || String(error));
    process.exitCode = 1;
});
