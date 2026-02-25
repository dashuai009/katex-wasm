import * as katex_wasm from "katex-wasm";
import katex from "katex";
import { Chart, LinearScale, PointElement, Legend, Title, Tooltip, ScatterController } from "chart.js";

// Register required components for Chart.js
Chart.register(LinearScale, PointElement, Legend, Title, Tooltip, ScatterController);

/**
 * 从文本文件中读取指定行号范围的内容
 * @param {string} filePath - 文件路径
 * @param {number} startLine - 起始行号（从1开始）
 * @param {number} endLine - 结束行号（包含）
 * @returns {Promise<string[]>} 返回读取到的行内容数组
 */
async function loadFormulasFromFile(filePath, startLine = 1, endLine = null) {
    try {
        const response = await fetch(filePath);
        
        if (!response.ok) {
            throw new Error(`Failed to load file: ${response.status} ${response.statusText}`);
        }
        
        const text = await response.text();
        const allLines = text.split('\n').filter(line => line.trim() !== '');
        
        // 参数校验
        const totalLines = allLines.length;
        const actualStart = Math.max(1, startLine);
        const actualEnd = endLine !== null ? Math.min(endLine, totalLines) : totalLines;
        
        if (actualStart > totalLines) {
            console.warn(`Start line ${startLine} exceeds total lines ${totalLines}, returning empty array`);
            return [];
        }
        
        if (actualStart > actualEnd) {
            console.warn(`Invalid line range: start=${startLine}, end=${endLine}`);
            return [];
        }
        
        // 行号从1开始，数组索引从0开始
        const selectedLines = allLines.slice(actualStart - 1, actualEnd);
        console.log(`Loaded ${selectedLines.length} formulas from ${filePath} (lines ${actualStart}-${actualEnd})`);
        
        return selectedLines;
    } catch (error) {
        console.error(`Error loading formulas from ${filePath}:`, error);
        throw error;
    }
}

// 配置参数（可以通过 URL 参数覆盖）
const urlParams = new URLSearchParams(window.location.search);
const CONFIG = {
    filePath: urlParams.get('file') || './formulas.txt',
    startLine: parseInt(urlParams.get('start')) || 1,
    endLine: urlParams.get('end') ? parseInt(urlParams.get('end')) : null
};

async function main() {
    console.log("CONFIG:", CONFIG);
    let math_wasm = document.getElementById("math-wasm");
    let math_katex = document.getElementById("math-katex");

    // Arrays to store performance data
    const perfData = [];

    // Load formulas from file
    let math_str;
    try {
        math_str = await loadFormulasFromFile(CONFIG.filePath, CONFIG.startLine, CONFIG.endLine);
        
        if (math_str.length === 0) {
            console.warn("No formulas loaded, using default formulas");
            math_str = ["E=mc^2", "x+y=z"];
        }
    } catch (error) {
        console.error("Failed to load formulas file, using fallback:", error);
        // Fallback to default formulas if file loading fails
        math_str = [
            "E=mc^2",
            "x+y=z",
            "a^2+b^2=c^2",
            "\\frac{1}{2}",
            "\\sqrt{x}"
        ];
    }

    for (let s of math_str) {
        // Measure katex-wasm rendering time
        let startTime = performance.now();
        try {
            let t = katex_wasm.renderToString(s, {displayMode: true, throwOnError: false, trust: true, maxSize: 200000, maxExpand: 1000});
            let endTime = performance.now();
            let wasmTime = endTime - startTime;
            
            let d = document.createElement("span");
            d.innerHTML = t;
            math_wasm.append(d);
            
            // Store performance data
            perfData.push({
                formula: s,
                length: s.length,
                wasmTime: wasmTime,
                katexTime: 0 // Will be updated later
            });
        } catch (error) {
            console.error("Error rendering formula with katex-wasm:", s, error);
            let errorMsg = document.createElement("div");
            errorMsg.textContent = "Error rendering formula: " + s;
            errorMsg.style.color = "red";
            math_wasm.append(errorMsg);
        }

        // Measure katex-js rendering time
        startTime = performance.now();
        try {
            let d2 = document.createElement("div");
            katex.render(s, d2, {displayMode: true, throwOnError: false, trust: true, max_size: 20000});
            let endTime = performance.now();
            let katexTime = endTime - startTime;
            
            math_katex.append(d2);
            
            // Update performance data
            const lastEntry = perfData[perfData.length - 1];
            if (lastEntry && lastEntry.formula === s) {
                lastEntry.katexTime = katexTime;
            }
        } catch (error) {
            console.error("Error rendering formula with katex:", s, error);
            let errorMsg = document.createElement("div");
            errorMsg.textContent = "Error rendering formula: " + s;
            errorMsg.style.color = "red";
            math_katex.append(errorMsg);
        }
    }

    // Create the scatter plot after rendering all formulas
    createScatterPlot(perfData);
}

function createScatterPlot(perfData) {
    // Create canvas element
    const chartContainer = document.createElement("div");
    chartContainer.style.marginTop = "50px";
    chartContainer.style.width = "800px";
    chartContainer.style.height = "600px";
    document.body.appendChild(chartContainer);
    
    const canvas = document.createElement("canvas");
    chartContainer.appendChild(canvas);
    
    // Prepare data for Chart.js
    const wasmData = perfData.map(d => ({ x: d.length, y: d.wasmTime }));
    const katexData = perfData.map(d => ({ x: d.length, y: d.katexTime }));
    
    // Create Chart.js scatter plot
    new Chart(canvas, {
        type: 'scatter',
        data: {
            datasets: [
                {
                    label: 'Rust/WASM',
                    data: wasmData,
                    backgroundColor: 'rgba(34, 139, 34, 0.7)',
                    borderColor: 'rgba(34, 139, 34, 1)',
                    pointRadius: 6,
                    pointHoverRadius: 8
                },
                {
                    label: 'JavaScript',
                    data: katexData,
                    backgroundColor: 'rgba(220, 20, 60, 0.7)',
                    borderColor: 'rgba(220, 20, 60, 1)',
                    pointRadius: 6,
                    pointHoverRadius: 8
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                title: {
                    display: true,
                    text: 'KaTeX Rendering Performance Comparison',
                    font: {
                        size: 18,
                        weight: 'bold'
                    }
                },
                legend: {
                    display: true,
                    position: 'top'
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const formula = perfData[context.dataIndex].formula;
                            return [
                                `${context.dataset.label}:`,
                                `  Formula Length: ${context.parsed.x}`,
                                `  Time: ${context.parsed.y.toFixed(2)}ms`,
                                `  Formula: ${formula.substring(0, 30)}${formula.length > 30 ? '...' : ''}`
                            ];
                        }
                    }
                }
            },
            scales: {
                x: {
                    type: 'linear',
                    position: 'bottom',
                    title: {
                        display: true,
                        text: 'Formula Length (characters)',
                        font: {
                            size: 14,
                            weight: 'bold'
                        }
                    },
                    grid: {
                        display: true,
                        color: 'rgba(0, 0, 0, 0.1)'
                    }
                },
                y: {
                    type: 'linear',
                    title: {
                        display: true,
                        text: 'Rendering Time (ms)',
                        font: {
                            size: 14,
                            weight: 'bold'
                        }
                    },
                    grid: {
                        display: true,
                        color: 'rgba(0, 0, 0, 0.1)'
                    },
                    beginAtZero: true
                }
            }
        }
    });
}

// 直接调用 main()，因为脚本在 body 末尾加载，DOM 已经准备好
main();