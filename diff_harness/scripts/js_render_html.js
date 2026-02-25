const fs = require('fs');
const payload = JSON.parse(fs.readFileSync(0, 'utf8'));
const katex = require('../../KaTeX/dist/katex.mjs');

try {
  const html = katex.renderToString(payload.expression, {
    displayMode: true,
    output: 'html',
    throwOnError: false,
    trust: true,
    strict: 'ignore',
  });
  process.stdout.write(html);
} catch (error) {
  process.stdout.write('JS_ERROR: ' + (error.message || String(error)));
}
