import assert from "node:assert/strict";
import * as wasm from "../../pkg/katex_wasm.js";
import state from "../../scripts/build-state.cjs";
state.assertFresh();
const options = {
  output: "html",
  throwOnError: false,
  strict: "ignore",
  trust: false,
};
assert.ok(!wasm.renderToString("x", options).includes("katex-display"));
options.displayMode = true;
assert.ok(wasm.renderToString("x", options).includes("katex-display"));
options.displayMode = false;
assert.ok(!wasm.renderToString("x", options).includes("katex-display"));
const style = new wasm.CssStyle();
style.background_color = "red";
assert.equal(style.backgroundColor, "red");
style.backgroundColor = "blue";
assert.equal(style.background_color, "blue");
style.height = "1em";
assert.equal(style.height, "1em");
style.height = undefined;
assert.equal(style.height, undefined);
style.free();
const escaped = wasm.renderToString("\\text{a\\&b}", options);
assert.ok(escaped.includes("a&amp;b"));
const trusted = wasm.renderToString("\\href{https://example.com}{x}", {
  ...options,
  trust: true,
});
assert.ok(trusted.includes('href="https://example.com"'));
const blocked = wasm.renderToString("\\href{https://example.com}{x}", options);
assert.ok(!blocked.includes('href="https://example.com"'));
assert.ok(
  !wasm.renderToString("\\url{https://example.com}", options).includes("href="),
);
console.log(
  "WASM API regressions passed: defaults, mutable options, style aliases, escaping, trust.",
);

const { canonicalHtml } = await import("../../tests/html_normalize.mjs");
assert.equal(
  canonicalHtml('<span style="top:1em;margin-right:2em"></span>'),
  canonicalHtml('<span style="margin-right:2em;top:1em"></span>'),
);
assert.notEqual(
  canonicalHtml('<span style="margin:1em;margin-left:2em"></span>'),
  canonicalHtml('<span style="margin-left:2em;margin:1em"></span>'),
);
assert.notEqual(
  canonicalHtml('<span style="color:red;color:blue"></span>'),
  canonicalHtml('<span style="color:blue;color:red"></span>'),
);
console.log(
  "HTML normalization preserves shorthand and duplicate declaration order.",
);

const { stats } = await import("../statistics.mjs");
assert.equal(stats([9, 1, 3, 5]).median, 4);
assert.equal(stats([9, 1, 3]).median, 3);
assert.equal(stats([]).median, null);

const { compareOutcomes } = await import("../behavior.mjs");
assert.ok(
  compareOutcomes(
    { kind: "return", html: '<span class="katex-error">bad</span>' },
    { kind: "return", html: '<span class="katex-error">bad</span>' },
  ).matched,
);
assert.ok(
  compareOutcomes(
    { kind: "throw", name: "ParseError", message: "KaTeX parse error: bad" },
    { kind: "throw", name: "Error", message: "ParseError: bad" },
  ).matched,
);
assert.ok(
  !compareOutcomes(
    { kind: "throw", name: "ParseError", message: "bad" },
    { kind: "throw", name: "RuntimeError", message: "bad" },
  ).matched,
);
assert.ok(
  !compareOutcomes(
    { kind: "throw", message: "bad" },
    { kind: "return", html: "bad" },
  ).matched,
);
assert.ok(
  !compareOutcomes(
    { kind: "return", html: "<span>x</span>" },
    { kind: "return", html: "<span>y</span>" },
  ).matched,
);
console.log(
  "Behavior checks accept matching errors and reject output/exception differences.",
);

assert.ok(
  !compareOutcomes(
    { kind: "return", html: "<span>1em</span>" },
    { kind: "return", html: "<span>1.0005em</span>" },
  ).matched,
);
assert.ok(
  compareOutcomes(
    { kind: "return", html: '<span style="height:1em">x</span>' },
    { kind: "return", html: '<span style="height:1.0005em">x</span>' },
  ).matched,
);

assert.ok(
  !compareOutcomes(
    { kind: "return", html: "<template>x</template>" },
    { kind: "return", html: "<template>y</template>" },
  ).matched,
);
