import * as wasm from "hello-wasm-pack";
import * as katex from "katex-wasm";
wasm.greet();

let s = "a^2+b^2=1";
console.log(katex)
let t = katex.renderToString(s,{displayMode: true, throwOnError: false});
console.log(t);
