import * as katex_wasm from "katex-wasm";
import katex from "katex";

const math_str = [
    "a^2+b^2=1",
    "E=mc^2",
    "\\\"{A}",
    "\\underleftarrow{AB} \\underrightarrow{AB} \\underleftrightarrow{AB} \\underlinesegment{AB} \\undergroup{AB} \\utilde{AB} \\xleftarrow{abc} \\xrightarrow{abc}  \\xLeftarrow{abc}  \\xRightarrow{abc} \\xleftrightarrow{abc}  \\xLeftrightarrow{abc}",
    "\\xhookleftarrow{abc}  \\xhookrightarrow{abc}  \\xmapsto{abc}  \\xrightharpoondown{abc}  \\xrightharpoonup{abc}  \\xleftharpoondown{abc}  \\xleftharpoonup{abc} \\xrightleftharpoons{abc}  \\xleftrightharpoons{abc}  \\xlongequal{abc} \\xtwoheadrightarrow{abc}  \\xtwoheadleftarrow{abc}  \\xtofrom{abc} \\xrightleftarrows{abc}  \\xrightequilibrium{abc}  \\xleftequilibrium{abc}",
    "\\cancel{5}",
    "\\frac{1}{2}",
    "\\overbrace{AB} \\underbrace{AB}"
]

let math_wasm = document.getElementById("math-wasm");
let math_katex = document.getElementById("math-katex")
for (let s of math_str) {
    let t = katex_wasm.renderToString(s, {displayMode: true, throwOnError: false});
    let d = document.createElement("span");
    d.innerHTML = t;
    math_wasm.append(d);

    let d2 = document.createElement("div");
    katex.render(s, d2, {displayMode: true, throwOnError: false});
    math_katex.append(d2);
}