const fs = require('fs');
const payload = JSON.parse(fs.readFileSync(0, 'utf8'));
const katex = require('../../KaTeX/dist/katex.mjs');

const settings = new katex.Settings({
  displayMode: true,
  output: 'html',
  throwOnError: false,
  trust: true,
  strict: 'ignore'
});

function stripNonSemantic(node) {
  if (node === null || node === undefined) return node;
  if (typeof node !== 'object') return node;
  if (Array.isArray(node)) return node.map(stripNonSemantic);
  const result = {};
  const keys = Object.keys(node).sort();
  for (const key of keys) {
    if (key === 'loc') continue;
    result[key] = stripNonSemantic(node[key]);
  }
  return result;
}

function stabilizeFloats(node) {
  if (node === null || node === undefined) return node;
  if (typeof node === 'number') {
    return Math.round(node * 1e8) / 1e8;
  }
  if (typeof node !== 'object') return node;
  if (Array.isArray(node)) return node.map(stabilizeFloats);
  const result = {};
  for (const [key, value] of Object.entries(node)) {
    result[key] = stabilizeFloats(value);
  }
  return result;
}

try {
  const tree = katex.parseTree(payload.expression, settings);
  const canonical = stabilizeFloats(stripNonSemantic(tree));
  process.stdout.write(JSON.stringify(canonical));
} catch (error) {
  process.stdout.write(JSON.stringify({ __error: error.message || String(error) }));
}
