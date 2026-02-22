import * as katex_wasm from "katex-wasm";
import katex from "katex";

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
        let t = katex_wasm.renderToString(s, {displayMode: true, throwOnError: false});
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
        katex.render(s, d2, {displayMode: true, throwOnError: false});
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

// Create scatter plot
function createScatterPlot() {
    // Create canvas for chart
    const chartContainer = document.createElement("div");
    chartContainer.id = "chart-container";
    chartContainer.style.marginTop = "50px";
    document.body.appendChild(chartContainer);
    
    // Create canvas element
    const canvas = document.createElement("canvas");
    canvas.id = "performance-chart";
    chartContainer.appendChild(canvas);
    
    // Simple scatter plot implementation
    const ctx = canvas.getContext('2d');
    canvas.width = 800;
    canvas.height = 600;
    
    // Find max values for scaling
    const maxLength = Math.max(...perfData.map(d => d.length));
    const maxTime = Math.max(
        ...perfData.map(d => d.wasmTime),
        ...perfData.map(d => d.katexTime)
    );
    
    // Set margins and chart area
    const margin = { top: 50, right: 50, bottom: 50, left: 50 };
    const width = canvas.width - margin.left - margin.right;
    const height = canvas.height - margin.top - margin.bottom;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw axes
    ctx.beginPath();
    ctx.moveTo(margin.left, margin.top);
    ctx.lineTo(margin.left, canvas.height - margin.bottom);
    ctx.lineTo(canvas.width - margin.right, canvas.height - margin.bottom);
    ctx.stroke();
    
    // Draw axis labels
    ctx.fillStyle = "black";
    ctx.font = "16px Arial";
    ctx.fillText("Formula Length", canvas.width / 2 - 50, canvas.height - 10);
    ctx.save();
    ctx.translate(15, canvas.height / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillText("Rendering Time (ms)", 0, 0);
    ctx.restore();
    
    // Draw data points
    perfData.forEach(point => {
        // Scale coordinates
        const x = margin.left + (point.length / maxLength) * width;
        const yWasm = canvas.height - margin.bottom - (point.wasmTime / maxTime) * height;
        const yKatex = canvas.height - margin.bottom - (point.katexTime / maxTime) * height;
        
        // Draw wasm points in green
        ctx.beginPath();
        ctx.arc(x, yWasm, 5, 0, Math.PI * 2);
        ctx.fillStyle = "green";
        ctx.fill();
        
        // Draw katex points in red
        ctx.beginPath();
        ctx.arc(x, yKatex, 5, 0, Math.PI * 2);
        ctx.fillStyle = "red";
        ctx.fill();
    });
    
    // Draw legend
    ctx.fillStyle = "green";
    ctx.fillRect(canvas.width - 150, 20, 15, 15);
    ctx.fillStyle = "black";
    ctx.font = "14px Arial";
    ctx.fillText("Rust/WASM", canvas.width - 130, 32);
    
    ctx.fillStyle = "red";
    ctx.fillRect(canvas.width - 150, 45, 15, 15);
    ctx.fillStyle = "black";
    ctx.fillText("JavaScript", canvas.width - 130, 57);
}

// Create the scatter plot after a short delay to ensure DOM is ready
setTimeout(createScatterPlot, 1000);