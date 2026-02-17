# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust WebAssembly port of KaTeX (katex-wasm) - a LaTeX math rendering library. It allows rendering mathematical expressions in web browsers using Rust-generated WASM code instead of JavaScript.

## Architecture

- **Core**: Built with `wasm-bindgen` to interface Rust code with JavaScript environment
- **Structure**: Contains a complete LaTeX parsing and rendering engine written in Rust
- **Web API**: Exposes functions for both direct DOM rendering and string-based rendering
- **Modules**: Includes lexer, parser, settings, build tree, symbol definitions, and DOM tree generation

Key modules:
- `katex.rs`: Main entry point with the three public functions (`render_to_dom_tree`, `render`, `render_to_string`)
- `settings/`: Configuration system compatible with original KaTeX options
- `parse/`: LaTeX parsing logic using a Parser module
- `build/`: Conversion from parse tree to DOM tree/markup
- `dom_tree/`: Virtual DOM implementation for output
- `symbols/`, `unicodeAccents/`, etc.: Supporting data and utilities

## Public API

The library exposes three main functions:
1. `render(expression, base_node, options)` - Renders LaTeX directly to DOM node (JS version for direct DOM manipulation)
2. `render_to_string_for_js(expression, settings)` - Creates HTML string from LaTeX expression (JS accessible)
3. `render_to_string(expression, settings)` - Pure Rust variant for string rendering

## Development Commands

### Build
```bash
wasm-pack build
```

### Test
```bash
# Run Rust unit tests
cargo test

# Run WASM-specific tests
wasm-pack test --headless --firefox

# Run specific tests
cargo test render_parity
```

### Develop Demo
```bash
wasm-pack build
cd demo
npm install
npm start
```

### Run Render Parity Tests
```bash
# Run render parity tests to check output matches JS KaTeX
cargo test parity

# Run with specific filter
PARITY_CASE_FILTER="frac" cargo test parity

# Run with exact case
PARITY_CASE_EXACT="\\frac{a}{b}" cargo test parity

# Force test assertion on failures
ASSERT_PARITY=1 cargo test parity
```

## Important Configuration Options

- `displayMode`: Whether to render in display mode (mathematical environments)
- `output`: Set output type (html, mathml, htmlAndMathml)
- `throwOnError`: Whether to throw errors for invalid LaTeX
- `errorColor`: Color for highlighting errors in expression
- `maxSize` and `maxExpand`: Safety limits on output size and macro expansion
- `trust`: Whether to allow potentially dangerous constructs