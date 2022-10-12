import * as katex_wasm from "katex-wasm";
import * as katex from "katex";

const math_str = [
    "a^2+b^2=1",
    "E=mc^2",
    "\\\"{A}"
]

let math_wasm = document.getElementById("math-wasm");
let math_katex = document.getElementById("math-katex")
for (let s of math_str) {
    let t = katex_wasm.renderToString(s, {displayMode: true, throwOnError: false});
    let d = document.createElement("div");
    d.innerHTML = t;
    math_wasm.append(d);

    let d2 = document.createElement("div");
    katex.render(s, d2, {displayMode: true, throwOnError: false});
    math_katex.append(d2);
}
