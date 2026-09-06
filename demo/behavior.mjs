import { parseFragment, serialize } from "parse5";
const sides = ["top", "right", "bottom", "left"];
const independent = new Set([
  "height",
  "width",
  "min-width",
  "max-width",
  "min-height",
  "max-height",
  "top",
  "right",
  "bottom",
  "left",
  "position",
  "color",
  "background-color",
  "vertical-align",
  "text-shadow",
  "font-size",
  "line-height",
  ...sides.flatMap((side) => [
    `margin-${side}`,
    `padding-${side}`,
    `border-${side}-width`,
    `border-${side}-style`,
    `border-${side}-color`,
  ]),
]);
function affectedProperties(property) {
  if (independent.has(property)) return [property];
  if (property === "margin" || property === "padding")
    return sides.map((side) => `${property}-${side}`);
  if (/^border-(width|style|color)$/.test(property))
    return sides.map((side) => `border-${side}-${property.slice(7)}`);
  return null;
}
function normalizeStyle(style) {
  // Split only at unquoted, top-level semicolons (e.g. keep calc()/URL values).
  const parts = [];
  let start = 0,
    depth = 0,
    quote = "",
    escaped = false;
  for (let i = 0; i < style.length; i++) {
    const c = style[i];
    if (escaped) {
      escaped = false;
      continue;
    }
    if (c === "\\") {
      escaped = true;
      continue;
    }
    if (quote) {
      if (c === quote) quote = "";
      continue;
    }
    if (c === '"' || c === "'") {
      quote = c;
      continue;
    }
    if (c === "(") depth++;
    else if (c === ")") depth--;
    else if (c === ";" && depth === 0) {
      parts.push(style.slice(start, i));
      start = i + 1;
    }
  }
  parts.push(style.slice(start));
  const used = new Set(),
    declarations = [];
  for (const part of parts.filter((part) => part.trim())) {
    const match = part.match(/^\s*([a-z-]+)\s*:\s*([\s\S]*?)\s*$/i);
    if (!match) return style;
    const property = match[1].toLowerCase(),
      affected = affectedProperties(property);
    // Never reorder unknown properties, duplicates or overlapping shorthands.
    if (!affected || affected.some((name) => used.has(name))) return style;
    affected.forEach((name) => used.add(name));
    declarations.push([property, match[2]]);
  }
  return declarations
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([key, value]) => `${key}:${value};`)
    .join("");
}
export function canonicalHtml(html) {
  const tree = parseFragment(html);
  function visit(node) {
    if (node.attrs) {
      for (const attr of node.attrs)
        if (attr.name === "style") attr.value = normalizeStyle(attr.value);
      node.attrs.sort((a, b) => a.name.localeCompare(b.name));
    }
    for (const child of node.childNodes || []) visit(child);
  }
  visit(tree);
  return serialize(tree);
}

// Preserve the existing Diff Harness tolerance and all non-numeric structure.
function numericAttributesMatch(left, right) {
  const a = [...left.matchAll(/(-?\d+(?:\.\d+)?)em\b/g)],
    b = [...right.matchAll(/(-?\d+(?:\.\d+)?)em\b/g)];
  if (a.length !== b.length) return false;
  let ai = 0,
    bi = 0;
  for (let i = 0; i < a.length; i++) {
    if (
      left.slice(ai, a[i].index) !== right.slice(bi, b[i].index) ||
      Math.abs(Number(a[i][1]) - Number(b[i][1])) > 0.001
    )
      return false;
    ai = a[i].index + a[i][0].length;
    bi = b[i].index + b[i][0].length;
  }
  return left.slice(ai) === right.slice(bi);
}
export function htmlMatchesWithinTolerance(left, right) {
  left = canonicalHtml(left);
  right = canonicalHtml(right);
  if (left === right) return true;
  function equal(a, b) {
    if (a.nodeName !== b.nodeName || a.namespaceURI !== b.namespaceURI)
      return false;
    if (a.nodeName === "#text") return a.value === b.value;
    if (a.nodeName === "#comment") return a.data === b.data;
    const aa = a.attrs || [],
      ba = b.attrs || [];
    if (aa.length !== ba.length) return false;
    for (let i = 0; i < aa.length; i++) {
      if (aa[i].name !== ba[i].name || aa[i].namespace !== ba[i].namespace)
        return false;
      const numeric = ["style", "width", "height"].includes(aa[i].name);
      if (
        numeric
          ? !numericAttributesMatch(aa[i].value, ba[i].value)
          : aa[i].value !== ba[i].value
      )
        return false;
    }
    if (Boolean(a.content) !== Boolean(b.content)) return false;
    if (a.content && !equal(a.content, b.content)) return false;
    const ac = a.childNodes || [],
      bc = b.childNodes || [];
    return (
      ac.length === bc.length && ac.every((child, i) => equal(child, bc[i]))
    );
  }
  return equal(parseFragment(left), parseFragment(right));
}

export function outcome(render, formula, settings) {
  try {
    return { kind: "return", html: render(formula, { ...settings }) };
  } catch (error) {
    return {
      kind: "throw",
      name: error?.name || "Error",
      message: error?.message || String(error),
    };
  }
}
export function compareOutcomes(reference, actual) {
  if (reference.kind !== actual.kind)
    return {
      matched: false,
      reason: "One renderer returned output while the other threw an exception",
    };
  if (reference.kind === "throw") {
    const normalize = (message) =>
      message.replace(/^(?:Error: )?(?:KaTeX parse error: |ParseError: )/, "");
    const category = (error) =>
      /^(?:KaTeX parse error: |ParseError: )/.test(error.message) ||
      error.name === "ParseError"
        ? "ParseError"
        : error.name || "Error";
    const matched =
      category(reference) === category(actual) &&
      normalize(reference.message) === normalize(actual.message);
    return {
      matched,
      reason: matched
        ? "Matching exceptions"
        : "Different exception types or messages",
    };
  }
  const matched =
    typeof reference.html === "string" &&
    typeof actual.html === "string" &&
    htmlMatchesWithinTolerance(reference.html, actual.html);
  return {
    matched,
    reason: matched
      ? "Matching output (including matching error output)"
      : "Different HTML structure, content, or numeric values",
  };
}
