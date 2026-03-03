# `katex-rs-cli` Coverage Guide

This document describes how to collect Rust code coverage by driving the project through `katex-rs-cli`.

## What Is Still Not Done

- A committed automation script for coverage collection has not been added yet.
- A baseline coverage report has not been checked into the repository.
- In restricted sandbox environments, `cargo` may fail with `Invalid cross-device link (os error 18)`. In that case, run the commands below in a normal host shell instead of the sandbox.

## Scope

This method measures the Rust code paths exercised when `katex-rs-cli` renders formulas from a text file.

It is useful for:

- smoke-testing parser and renderer paths with real formula fixtures
- checking whether changes increase or decrease exercised code
- generating a local HTML coverage report before refactors

It does not replace browser-side tests or the JS vs Rust diff harness.

## Environment Setup

### Required tools

- Rust toolchain (`rustup`, `cargo`, `rustc`)
- The `llvm-tools-preview` Rust component

Install the LLVM tools component once:

```bash
rustup component add llvm-tools-preview
```

### Verify the CLI builds

Before collecting coverage, make sure the CLI runs:

```bash
cargo run --bin katex-rs-cli -- --help
```

If that command fails in a sandbox with `Invalid cross-device link`, rerun it in a normal shell outside the sandbox.

### Locate the LLVM coverage binaries

`llvm-profdata` and `llvm-cov` are shipped inside the Rust toolchain after `llvm-tools-preview` is installed.

```bash
rustc --print sysroot
```

On this machine, the tool location is:

```bash
$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/
```

The two binaries used below are:

- `llvm-profdata`
- `llvm-cov`

## Coverage Collection Process

### 1. Prepare output directories

```bash
mkdir -p coverage
mkdir -p target/coverage
```

Optional cleanup before a fresh run:

```bash
rm -f coverage/*.profraw
rm -f coverage/*.profdata
rm -rf coverage/html
```

### 2. Run an instrumented build through `katex-rs-cli`

This example renders the first 200 lines from the shared fixture file:

```bash
RUSTFLAGS="-Cinstrument-coverage" \
CARGO_INCREMENTAL=0 \
LLVM_PROFILE_FILE="coverage/katex-rs-cli-%p-%m.profraw" \
CARGO_TARGET_DIR="target/coverage" \
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 200
```

Notes:

- `RUSTFLAGS="-Cinstrument-coverage"` enables LLVM coverage instrumentation.
- `CARGO_INCREMENTAL=0` avoids unsupported incremental coverage artifacts.
- `LLVM_PROFILE_FILE=...` writes one raw profile per process.
- `CARGO_TARGET_DIR=target/coverage` keeps coverage builds separate from normal builds.

To exercise more code paths, replace the fixture or increase the processed line range.

For example:

```bash
RUSTFLAGS="-Cinstrument-coverage" \
CARGO_INCREMENTAL=0 \
LLVM_PROFILE_FILE="coverage/katex-rs-cli-%p-%m.profraw" \
CARGO_TARGET_DIR="target/coverage" \
cargo run --bin katex-rs-cli -- tests/fixtures/im2latex_formulas.lst 1 1000
```

### 3. Merge raw profiles into a `.profdata` file

Set the toolchain root first:

```bash
SYSROOT="$(rustc --print sysroot)"
```

Then merge:

```bash
"$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata" \
merge -sparse coverage/katex-rs-cli-*.profraw \
-o coverage/katex-rs-cli.profdata
```

### 4. Print a terminal coverage summary

```bash
"$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov" \
report target/coverage/debug/katex-rs-cli \
--instr-profile=coverage/katex-rs-cli.profdata \
--ignore-filename-regex='(/.cargo/registry)|(/rustc/)'
```

This prints line, function, and region coverage in the terminal.

### 5. Generate an HTML coverage report

```bash
"$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov" \
show target/coverage/debug/katex-rs-cli \
--instr-profile=coverage/katex-rs-cli.profdata \
--format=html \
--output-dir=coverage/html \
--ignore-filename-regex='(/.cargo/registry)|(/rustc/)'
```

After that, open:

- `coverage/html/index.html`

## Recommended Test Inputs

Use progressively larger inputs depending on what you want to inspect.

### Fast smoke test

```bash
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt 1 20
```

### Parser and renderer path sweep

```bash
cargo run --bin katex-rs-cli -- tests/fixtures/formulas.txt
```

### Wider corpus

```bash
cargo run --bin katex-rs-cli -- tests/fixtures/im2latex_formulas.lst 1 1000
```

For very large runs, start with a smaller range first so you can confirm the profile files are being generated correctly.

## Interpreting Results

- Coverage will reflect only paths reached by the formulas you feed into the CLI.
- Parse errors still count as executed code and can improve parser-path coverage.
- A higher percentage is not automatically better if the input corpus is low quality.

When comparing two changes, keep the same input file and line range so the report is comparable.

## Troubleshooting

### `Invalid cross-device link (os error 18)`

This has been observed in restricted sandbox execution. Use a normal host shell, or run commands with elevated permissions outside the sandbox.

### `llvm-profdata` or `llvm-cov` not found

Install the Rust component again:

```bash
rustup component add llvm-tools-preview
```

Then verify the path returned by:

```bash
rustc --print sysroot
```

### No `.profraw` files generated

Check:

- `LLVM_PROFILE_FILE` is set
- the `coverage/` directory exists
- the CLI command actually ran to completion

### Report includes too much third-party code

Keep the ignore regex:

```text
(/.cargo/registry)|(/rustc/)
```

You can further narrow it later if you only want project-local files.
