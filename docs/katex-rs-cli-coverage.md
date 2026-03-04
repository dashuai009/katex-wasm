# `katex-rs-cli` Coverage Guide

This document describes how to collect Rust code coverage by driving the project through `katex-rs-cli`.

## current report

```
Filename                                                                                                                   Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/thread_local/native/mod.rs          12                 5    58.33%           3                 1    66.67%           8                 3    62.50%           0                 0         -
katex-wasm/src/Lexer.rs                                                                                                        119                10    91.60%           6                 1    83.33%          94                 5    94.68%           0                 0         -
katex-wasm/src/Namespace.rs                                                                                                     96                14    85.42%           7                 0   100.00%          74                 9    87.84%           0                 0         -
katex-wasm/src/Options/mod.rs                                                                                                  223                26    88.34%          25                 2    92.00%         191                21    89.01%           0                 0         -
katex-wasm/src/Options/types.rs                                                                                                 32                 9    71.88%           4                 0   100.00%          26                 6    76.92%           0                 0         -
katex-wasm/src/Parser.rs                                                                                                      1261               353    72.01%          32                 3    90.62%         903               239    73.53%           0                 0         -
katex-wasm/src/Style.rs                                                                                                         37                 0   100.00%           8                 0   100.00%          28                 0   100.00%           0                 0         -
katex-wasm/src/bin/katex_rs_cli.rs                                                                                             239                50    79.08%           9                 1    88.89%         152                43    71.71%           0                 0         -
katex-wasm/src/build/HTML.rs                                                                                                   602                72    88.04%          16                 0   100.00%         352                37    89.49%           0                 0         -
katex-wasm/src/build/common.rs                                                                                                 797               153    80.80%          15                 2    86.67%         496               100    79.84%           0                 0         -
katex-wasm/src/build/mathML.rs                                                                                                 259               259     0.00%           9                 9     0.00%         180               180     0.00%           0                 0         -
katex-wasm/src/build/mod.rs                                                                                                    104                60    42.31%           3                 1    66.67%          72                41    43.06%           0                 0         -
katex-wasm/src/define/environments/array.rs                                                                                   1032               351    65.99%          20                 6    70.00%         742               255    65.63%           0                 0         -
katex-wasm/src/define/functions/def_spec/accent.rs                                                                             245                10    95.92%           4                 1    75.00%         227                 5    97.80%           0                 0         -
katex-wasm/src/define/functions/def_spec/accentunder.rs                                                                         92                92     0.00%           3                 3     0.00%          74                74     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/arrow.rs                                                                              210               210     0.00%           4                 4     0.00%         179               179     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/assembleSupSub.rs                                                                     170                 1    99.41%           1                 0   100.00%         182                 1    99.45%           0                 0         -
katex-wasm/src/define/functions/def_spec/char.rs                                                                                49                49     0.00%           2                 2     0.00%          31                31     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/color.rs                                                                              105                84    20.00%           5                 4    20.00%          78                65    16.67%           0                 0         -
katex-wasm/src/define/functions/def_spec/cr.rs                                                                                  85                32    62.35%           3                 1    66.67%          43                13    69.77%           0                 0         -
katex-wasm/src/define/functions/def_spec/def.rs                                                                                240               187    22.08%           7                 6    14.29%         160               120    25.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/delimsizing.rs                                                                        477               215    54.93%          36                27    25.00%         351               163    53.56%           0                 0         -
katex-wasm/src/define/functions/def_spec/enclose.rs                                                                            466               346    25.75%           7                 5    28.57%         362               270    25.41%           0                 0         -
katex-wasm/src/define/functions/def_spec/environment.rs                                                                        115                24    79.13%           2                 0   100.00%          95                23    75.79%           0                 0         -
katex-wasm/src/define/functions/def_spec/font.rs                                                                                93                26    72.04%           5                 2    60.00%          76                24    68.42%           0                 0         -
katex-wasm/src/define/functions/def_spec/genfrac.rs                                                                            372                60    83.87%           6                 1    83.33%         288                42    85.42%           0                 0         -
katex-wasm/src/define/functions/def_spec/hbox.rs                                                                                32                 2    93.75%           3                 1    66.67%          25                 2    92.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/horiz_brace.rs                                                                        186                51    72.58%           3                 1    66.67%         191                65    65.97%           0                 0         -
katex-wasm/src/define/functions/def_spec/href.rs                                                                                83                83     0.00%           5                 5     0.00%          56                56     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/html.rs                                                                               127               127     0.00%           4                 4     0.00%          74                74     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/htmlmathml.rs                                                                          41                 9    78.05%           3                 1    66.67%          30                 7    76.67%           0                 0         -
katex-wasm/src/define/functions/def_spec/includegraphics.rs                                                                    222               222     0.00%           5                 5     0.00%         144               144     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/kern.rs                                                                                55                16    70.91%           3                 1    66.67%          41                 9    78.05%           0                 0         -
katex-wasm/src/define/functions/def_spec/lap.rs                                                                                154                54    64.94%           3                 1    66.67%          88                32    63.64%           0                 0         -
katex-wasm/src/define/functions/def_spec/math.rs                                                                                35                 3    91.43%           2                 1    50.00%          25                 2    92.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/mathchoice.rs                                                                          66                13    80.30%           4                 1    75.00%          42                 8    80.95%           0                 0         -
katex-wasm/src/define/functions/def_spec/mclass.rs                                                                             171               123    28.07%           5                 3    40.00%         101                62    38.61%           0                 0         -
katex-wasm/src/define/functions/def_spec/op.rs                                                                                 399               151    62.16%           9                 2    77.78%         319                83    73.98%           0                 0         -
katex-wasm/src/define/functions/def_spec/operatorname.rs                                                                       105               105     0.00%           4                 4     0.00%          95                95     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/ordgroup.rs                                                                            41                19    53.66%           3                 2    33.33%          28                14    50.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/overline.rs                                                                            66                 2    96.97%           3                 1    66.67%          63                 2    96.83%           0                 0         -
katex-wasm/src/define/functions/def_spec/phantom.rs                                                                            156                69    55.77%           9                 5    44.44%         128                66    48.44%           0                 0         -
katex-wasm/src/define/functions/def_spec/pmb.rs                                                                                 42                42     0.00%           3                 3     0.00%          31                31     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/raisebox.rs                                                                            51                 2    96.08%           3                 1    66.67%          37                 2    94.59%           0                 0         -
katex-wasm/src/define/functions/def_spec/relax.rs                                                                                9                 0   100.00%           1                 0   100.00%          12                 0   100.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/rule.rs                                                                                86                 2    97.67%           4                 1    75.00%          39                 2    94.87%           0                 0         -
katex-wasm/src/define/functions/def_spec/sizing.rs                                                                              83                 7    91.57%           6                 1    83.33%          53                 8    84.91%           0                 0         -
katex-wasm/src/define/functions/def_spec/smash.rs                                                                              110               110     0.00%           3                 3     0.00%          96                96     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/sqrt.rs                                                                               190                80    57.89%           3                 1    66.67%         129                49    62.02%           0                 0         -
katex-wasm/src/define/functions/def_spec/styling.rs                                                                            100                51    49.00%           4                 2    50.00%          60                29    51.67%           0                 0         -
katex-wasm/src/define/functions/def_spec/supsub.rs                                                                             275                18    93.45%           4                 2    50.00%         206                15    92.72%           0                 0         -
katex-wasm/src/define/functions/def_spec/symbols_op.rs                                                                          62                46    25.81%           3                 2    33.33%          35                24    31.43%           0                 0         -
katex-wasm/src/define/functions/def_spec/symbols_ord.rs                                                                        110                94    14.55%           6                 4    33.33%          67                50    25.37%           0                 0         -
katex-wasm/src/define/functions/def_spec/symbols_spacing.rs                                                                    116                32    72.41%           3                 2    33.33%          63                18    71.43%           0                 0         -
katex-wasm/src/define/functions/def_spec/tag.rs                                                                                 60                60     0.00%           3                 3     0.00%          43                43     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/text.rs                                                                               102                28    72.55%           4                 1    75.00%          54                 9    83.33%           0                 0         -
katex-wasm/src/define/functions/def_spec/underline.rs                                                                           69                 6    91.30%           3                 1    66.67%          63                 6    90.48%           0                 0         -
katex-wasm/src/define/functions/def_spec/vcenter.rs                                                                             56                56     0.00%           3                 3     0.00%          35                35     0.00%           0                 0         -
katex-wasm/src/define/functions/def_spec/verb.rs                                                                                67                67     0.00%           4                 4     0.00%          36                36     0.00%           0                 0         -
katex-wasm/src/define/functions/public.rs                                                                                       79                 4    94.94%          20                 1    95.00%          83                 4    95.18%           0                 0         -
katex-wasm/src/define/macros/macro_expander.rs                                                                                 520               127    75.58%          26                 4    84.62%         364                93    74.45%           0                 0         -
katex-wasm/src/define/macros/macro_map.rs                                                                                      440               102    76.82%          17                 4    76.47%         411                63    84.67%           0                 0         -
katex-wasm/src/define/macros/public.rs                                                                                           4                 4     0.00%           1                 1     0.00%           3                 3     0.00%           0                 0         -
katex-wasm/src/delimiter.rs                                                                                                    839                70    91.66%          16                 0   100.00%         603                74    87.73%           0                 0         -
katex-wasm/src/dom_tree/anchor.rs                                                                                               54                54     0.00%           7                 7     0.00%          44                44     0.00%           0                 0         -
katex-wasm/src/dom_tree/css_style.rs                                                                                             9                 6    33.33%           2                 1    50.00%           6                 3    50.00%           0                 0         -
katex-wasm/src/dom_tree/document_fragment.rs                                                                                    51                21    58.82%           6                 2    66.67%          38                11    71.05%           0                 0         -
katex-wasm/src/dom_tree/img.rs                                                                                                  74                74     0.00%           6                 6     0.00%          41                41     0.00%           0                 0         -
katex-wasm/src/dom_tree/line_node.rs                                                                                            80                80     0.00%           8                 8     0.00%          42                42     0.00%           0                 0         -
katex-wasm/src/dom_tree/path_node.rs                                                                                            69                40    42.03%           9                 5    44.44%          41                20    51.22%           0                 0         -
katex-wasm/src/dom_tree/span.rs                                                                                                 48                 7    85.42%           8                 2    75.00%          45                 6    86.67%           0                 0         -
katex-wasm/src/dom_tree/svg_node.rs                                                                                            119                78    34.45%          21                19     9.52%          90                60    33.33%           0                 0         -
katex-wasm/src/dom_tree/symbol_node.rs                                                                                         155                63    59.35%           9                 3    66.67%         101                33    67.33%           0                 0         -
katex-wasm/src/dom_tree/utils.rs                                                                                                 2                 0   100.00%           1                 0   100.00%           1                 0   100.00%           0                 0         -
katex-wasm/src/katex.rs                                                                                                        108                67    37.96%           8                 4    50.00%          55                30    45.45%           0                 0         -
katex-wasm/src/mathML_tree/math_node.rs                                                                                        109               109     0.00%          11                11     0.00%          73                73     0.00%           0                 0         -
katex-wasm/src/mathML_tree/public.rs                                                                                            98                98     0.00%           4                 4     0.00%          69                69     0.00%           0                 0         -
katex-wasm/src/mathML_tree/space_node.rs                                                                                        86                86     0.00%           9                 9     0.00%          55                55     0.00%           0                 0         -
katex-wasm/src/mathML_tree/text_node.rs                                                                                         43                43     0.00%           9                 9     0.00%          29                29     0.00%           0                 0         -
katex-wasm/src/metrics/fontMetricsData.rs                                                                                      152                98    35.53%           2                 1    50.00%          44                23    47.73%           0                 0         -
katex-wasm/src/metrics/mod.rs                                                                                                   46                 7    84.78%           2                 0   100.00%          30                 4    86.67%           0                 0         -
katex-wasm/src/parse/mod.rs                                                                                                     21                 7    66.67%           3                 2    33.33%          16                 5    68.75%           0                 0         -
katex-wasm/src/parse_error.rs                                                                                                   55                 1    98.18%           2                 0   100.00%          31                 1    96.77%           0                 0         -
katex-wasm/src/parse_node/mod.rs                                                                                                33                10    69.70%           2                 0   100.00%          18                 4    77.78%           0                 0         -
katex-wasm/src/parse_node/types.rs                                                                                              34                 5    85.29%           5                 0   100.00%          32                 5    84.38%           0                 0         -
katex-wasm/src/settings/mod.rs                                                                                                 291               210    27.84%          34                12    64.71%         207               119    42.51%           0                 0         -
katex-wasm/src/settings/settings_types.rs                                                                                       32                18    43.75%           4                 1    75.00%          26                14    46.15%           0                 0         -
katex-wasm/src/sourceLocation.rs                                                                                                46                 5    89.13%           9                 1    88.89%          46                 4    91.30%           0                 0         -
katex-wasm/src/spacingData.rs                                                                                                   32                 0   100.00%           2                 0   100.00%          18                 0   100.00%           0                 0         -
katex-wasm/src/stretchy.rs                                                                                                     349               134    61.60%           5                 1    80.00%         215                68    68.37%           0                 0         -
katex-wasm/src/svgGeometry.rs                                                                                                  140                23    83.57%          11                 1    90.91%         111                13    88.29%           0                 0         -
katex-wasm/src/symbols/mathS.rs                                                                                               1688                 0   100.00%           1                 0   100.00%         576                 0   100.00%           0                 0         -
katex-wasm/src/symbols/mod.rs                                                                                                   75                56    25.33%           3                 2    33.33%          43                34    20.93%           0                 0         -
katex-wasm/src/symbols/public.rs                                                                                               109                78    28.44%           7                 5    28.57%          94                60    36.17%           0                 0         -
katex-wasm/src/symbols/textS.rs                                                                                                365                 0   100.00%           1                 0   100.00%         147                 0   100.00%           0                 0         -
katex-wasm/src/token.rs                                                                                                         21                 9    57.14%           3                 2    33.33%          36                20    44.44%           0                 0         -
katex-wasm/src/tree.rs                                                                                                          14                 0   100.00%           4                 0   100.00%          12                 0   100.00%           0                 0         -
katex-wasm/src/types.rs                                                                                                         95                50    47.37%           8                 4    50.00%          73                41    43.84%           0                 0         -
katex-wasm/src/unicodeAccents.rs                                                                                                 5                 5     0.00%           1                 1     0.00%           5                 5     0.00%           0                 0         -
katex-wasm/src/unicodeScripts.rs                                                                                                20                 2    90.00%           2                 0   100.00%          16                 2    87.50%           0                 0         -
katex-wasm/src/unicodeSysmbols.rs                                                                                                7                 2    71.43%           1                 0   100.00%           5                 1    80.00%           0                 0         -
katex-wasm/src/units.rs                                                                                                         70                 2    97.14%           4                 0   100.00%          42                 1    97.62%           0                 0         -
katex-wasm/src/utils.rs                                                                                                         55                10    81.82%           6                 2    66.67%          43                11    74.42%           0                 0         -
katex-wasm/src/wide_character.rs                                                                                                31                31     0.00%           1                 1     0.00%          16                16     0.00%           0                 0         -
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                                                                                        17832              6344    64.42%         691               290    58.03%       12069              4327    64.15%           0                 0         -
```

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
