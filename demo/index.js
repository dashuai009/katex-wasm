import * as katex_wasm from "katex-wasm";
import katex from "katex";
import { Chart, LinearScale, PointElement, Legend, Title, Tooltip, ScatterController } from "chart.js";

// Register required components for Chart.js
Chart.register(LinearScale, PointElement, Legend, Title, Tooltip, ScatterController);

const math_str = [
    // Basic arithmetic
    "E=mc^2",
    "x+y=z",
    "a^2+b^2=c^2",
    "\\frac{1}{2}",
    "\\frac{a+b}{c+d}",
    "\\sqrt{x}",
    "\\sqrt{a^2+b^2}",
    "\\sqrt[3]{x^2+y^2}",

    // Trigonometric and logs
    "\\sin x",
    "\\cos^2\\theta+\\sin^2\\theta=1",
    "\\tan\\left(\\frac\\pi4\\right)=1",
    "\\log x",
    "\\ln(1+x)",
    "\\exp\\left(-\\frac{x^2}{2}\\right)",

    // Sums, products, limits
    "\\sum_{i=1}^{n} i",
    "\\sum_{k=0}^{\\infty} \\frac{x^k}{k!}",
    "\\prod_{i=1}^{n} i",
    "\\lim_{x\\to 0} \\frac{\\sin x}{x}",
    "\\int_0^1 x^2\\,dx",
    "\\int_{-\\infty}^{\\infty} e^{-x^2}\\,dx",
    "\\oint_C f(z)\\,dz",

    // Matrices and arrays
    "\\begin{pmatrix}a&b\\\\c&d\\end{pmatrix}",
    "\\begin{bmatrix}1&0\\\\0&1\\end{bmatrix}",
    "\\begin{vmatrix}a&b\\\\c&d\\end{vmatrix}",
    "\\left[\\begin{array}{cc}x&y\\\\z&w\\end{array}\\right]",

    // Relations and symbols
    "x\\in A",
    "A\\subseteq B",
    "f:A\\to B",
    "\\forall x\\in\\mathbb{R},\\exists y\\ge 0",
    "\\alpha+\\beta=\\gamma",
    "\\nabla\\cdot\\vec{F}=0",
    "\\partial_x u + \\partial_y u = 0",

    // Styling/macros commonly supported by KaTeX
    "\\mathbf{F}=m\\mathbf{a}",
    "\\mathit{abc}",
    "\\mathrm{d}x",
    "\\text{speed}=\\frac{\\text{distance}}{\\text{time}}",
    "\\color{blue}{x+y}",
    "\\underline{x+y}",
    "\\overline{z}",
    "\\hat{x}",
    "\\vec{v}",

    // Brackets and sizing
    "\\left(\\frac{a}{b}\\right)",
    "\\left\\{x\\in\\mathbb{R}\\mid x>0\\right\\}",
    "\\left\\langle x,y\\right\\rangle",
    "\\bigl( x \\bigr)",
    "\\Bigl[ y \\Bigr]",

    // Mixed hard-ish cases
    "\\frac{\\sqrt{1+x}-1}{x}",
    "\\sum_{i=1}^{n}\\frac{1}{i^2}",
    "\\int_0^{2\\pi}\\sin(nx)\\,dx",
    "\\left(\\sum_{i=1}^{n} i\\right)^2",
    "\\frac{\\partial^2 u}{\\partial x^2}+\\frac{\\partial^2 u}{\\partial y^2}=0",
    "\\binom{n}{k}",
    "\\left|\\frac{a+b}{c+d}\\right|",
    "\\operatorname{Var}(X)=\\mathbb{E}[X^2]-\\mathbb{E}[X]^2",
    "\\Pr(A\\mid B)=\\frac{\\Pr(A\\cap B)}{\\Pr(B)}",
    "\\left\\lVert x \\right\\rVert_2"
]

let math_wasm = document.getElementById("math-wasm");
let math_katex = document.getElementById("math-katex")

// Arrays to store performance data
const perfData = [];

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

// Create scatter plot using Chart.js
function createScatterPlot() {
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

// Create the scatter plot after a short delay to ensure DOM is ready
setTimeout(createScatterPlot, 1000);