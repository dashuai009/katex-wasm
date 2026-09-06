import {
  engines,
  render,
  benchmark,
  memoryBytes,
  settings,
} from "./benchmark.js";
import {
  Chart,
  LinearScale,
  PointElement,
  Legend,
  Title,
  Tooltip,
  ScatterController,
} from "chart.js";
Chart.register(
  LinearScale,
  PointElement,
  Legend,
  Title,
  Tooltip,
  ScatterController,
);
const $ = (id) => document.getElementById(id);
const colors = [
  "rgba(34,139,34,0.7)",
  "rgba(220,20,60,0.7)",
  "rgba(30,100,210,0.7)",
];
let chart;
const formulasReady = fetch("public/formulas.txt").then(async (response) => {
  if (!response.ok)
    throw Error(`Failed to load formulas.txt: ${response.status}`);
  return (await response.text()).trimEnd().split("\n");
});
function showFormulas(formulas) {
  $("formulas").replaceChildren();
  for (const engine of engines) {
    const title = document.createElement("div");
    title.className = "engine-heading";
    title.textContent = engine.name;
    $("formulas").append(title);
  }
  formulas.forEach((formula, index) =>
    engines.forEach((engine) => {
      const cell = document.createElement("div");
      cell.className = "formula";
      cell.dataset.engine = engine.name;
      cell.title = `Line ${index + 1}: ${formula}`;
      const output = document.createElement("div");
      try {
        output.innerHTML = engine.render(formula, { ...settings });
      } catch (error) {
        output.className = "error";
        output.textContent = String(error);
      }
      cell.append(output);
      $("formulas").append(cell);
    }),
  );
}
function show(result) {
  $("summary").querySelector("tbody").replaceChildren();
  engines.forEach((engine) => {
    const data = result.aggregateSummary[engine.id];
    const row = document.createElement("tr");
    for (const value of [
      engine.name,
      `${data.batch.median.toFixed(3)} ms`,
      `${data.mean.toFixed(3)} ms`,
      `${data.p95.toFixed(3)} ms`,
      engine.id === "wasm"
        ? `${result.behavior.matchedCount} / ${result.behavior.total} match JS`
        : engine.id === "katex"
          ? "Reference"
          : result.exceptions.katexRs.length
            ? `Independent comparison; ${result.exceptions.katexRs.length} exceptions`
            : "Independent comparison",
    ]) {
      const td = document.createElement("td");
      td.textContent = value;
      row.append(td);
    }
    $("summary").querySelector("tbody").append(row);
  });
  chart?.destroy();
  chart = new Chart($("chart"), {
    type: "scatter",
    data: {
      datasets: engines.map((engine, index) => ({
        label: engine.name,
        backgroundColor: colors[index],
        pointRadius: 5,
        pointHoverRadius: 8,
        data: result.perFormula.map((item) => ({
          x: item.formula.length,
          y: item.times[engine.id].mean,
          formula: item.formula,
        })),
      })),
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      animation: false,
      plugins: {
        title: {
          display: true,
          text: "KaTeX Rendering Performance Comparison",
          font: { size: 18, weight: "bold" },
        },
        legend: { position: "top" },
        tooltip: {
          callbacks: {
            label: (context) => [
              `${context.dataset.label}: ${context.parsed.y.toFixed(3)} ms`,
              `Length: ${context.parsed.x}`,
              context.raw.formula,
            ],
          },
        },
      },
      scales: {
        x: {
          type: "linear",
          title: { display: true, text: "Formula Length (characters)" },
        },
        y: {
          title: { display: true, text: "HTML Generation Time (ms)" },
          beginAtZero: true,
        },
      },
    },
  });
}
async function run() {
  window.__PERF_DONE__ = false;
  try {
    const all = await formulasReady;
    const params = new URLSearchParams(location.search),
      start = Number(params.get("start") || 1),
      end = Number(params.get("end") || all.length);
    if (
      !Number.isInteger(start) ||
      !Number.isInteger(end) ||
      start < 1 ||
      end < start ||
      end > all.length
    )
      throw Error("Invalid formula range");
    const formulas = all.slice(start - 1, end);
    const config = {
      repeat: Number(params.get("repeat") || 6),
      warmup: Number(params.get("warmup") ?? 2),
      mode: "compute",
    };
    if (
      !Number.isInteger(config.repeat) ||
      config.repeat < 1 ||
      !Number.isInteger(config.warmup) ||
      config.warmup < 0 ||
      config.mode !== "compute"
    )
      throw Error("Invalid benchmark settings");
    const result = await benchmark(formulas, config);
    Object.assign(result.config, { start, end, file: "public/formulas.txt" });
    show(result);
    window.__PERF_RESULT__ = result;
    showFormulas(formulas);
  } catch (error) {
    console.error(error);
    window.__PERF_RESULT__ = { error: String(error) };
    throw error;
  } finally {
    window.__PERF_DONE__ = true;
  }
}
$("versions").textContent = engines
  .map(
    (e) =>
      `${e.name} ${e.version}${e.revision ? " · " + e.revision.slice(0, 7) : ""}`,
  )
  .join(" / ");
window.__WASM_MEMORY_BYTES__ = memoryBytes;
window.__RUN_BENCHMARK__ = run;
window.__ENGINES__ = engines;
window.__RENDER_CHECK__ = render;
if (new URLSearchParams(location.search).get("autorun") !== "0")
  run().catch(console.error);
else formulasReady.catch(console.error);
