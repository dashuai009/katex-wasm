#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  scripts/katex-rs-cli-coverage.sh <formulas.txt> [--profraw-dir <dir>] [--merge-profraw <path> ...]

Examples:
  scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas.txt
  scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas.txt --profraw-dir coverage/profraw/file-a
  scripts/katex-rs-cli-coverage.sh tests/fixtures/formulas.txt --profraw-dir coverage/profraw/file-b --merge-profraw coverage/profraw/file-a

Environment overrides:
  COVERAGE_DIR=coverage                Output directory for report artifacts
  PROFRAW_DIR=coverage/profraw         Directory for current-run .profraw files
  CARGO_COVERAGE_TARGET_DIR=target/coverage
                                       Cargo target directory for instrumented build
  COVERAGE_IGNORE_REGEX='(/.cargo/registry)|(/rustc/)'
                                       Regex passed to llvm-cov --ignore-filename-regex
  PROFILE_PREFIX=katex-rs-cli          Prefix used for .profraw/.profdata file names
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

FORMULAS_FILE=""
PROFRAW_DIR_FROM_ARG=""
MERGE_PROFRAW_SPECS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profraw-dir)
      if [[ $# -lt 2 ]]; then
        echo "Error: --profraw-dir requires a value" >&2
        usage
        exit 1
      fi
      PROFRAW_DIR_FROM_ARG="$2"
      shift 2
      ;;
    --merge-profraw)
      if [[ $# -lt 2 ]]; then
        echo "Error: --merge-profraw requires a value" >&2
        usage
        exit 1
      fi
      MERGE_PROFRAW_SPECS+=("$2")
      shift 2
      ;;
    --*)
      echo "Error: unknown option: $1" >&2
      usage
      exit 1
      ;;
    *)
      if [[ -n "$FORMULAS_FILE" ]]; then
        echo "Error: only one formulas file is allowed, got extra argument: $1" >&2
        usage
        exit 1
      fi
      FORMULAS_FILE="$1"
      shift
      ;;
  esac
done

if [[ -z "$FORMULAS_FILE" ]]; then
  usage
  exit 1
fi

if [[ ! -f "$FORMULAS_FILE" ]]; then
  echo "Error: formulas file not found: $FORMULAS_FILE" >&2
  exit 1
fi

# if [[ "$FORMULAS_FILE" != *.txt ]]; then
#   echo "Error: formulas input must be a .txt file: $FORMULAS_FILE" >&2
#   exit 1
# fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

COVERAGE_DIR="${COVERAGE_DIR:-coverage}"
PROFRAW_DIR="${PROFRAW_DIR:-$COVERAGE_DIR/profraw}"
if [[ -n "$PROFRAW_DIR_FROM_ARG" ]]; then
  PROFRAW_DIR="$PROFRAW_DIR_FROM_ARG"
fi
CARGO_COVERAGE_TARGET_DIR="${CARGO_COVERAGE_TARGET_DIR:-target/coverage}"
COVERAGE_IGNORE_REGEX="${COVERAGE_IGNORE_REGEX:-(/.cargo/registry)|(/rustc/)}"
PROFILE_PREFIX="${PROFILE_PREFIX:-katex-rs-cli}"

SYSROOT="$(rustc --print sysroot)"
HOST_TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"
LLVM_BIN_DIR="$SYSROOT/lib/rustlib/$HOST_TRIPLE/bin"
LLVM_PROFDATA="$LLVM_BIN_DIR/llvm-profdata"
LLVM_COV="$LLVM_BIN_DIR/llvm-cov"

if [[ ! -x "$LLVM_PROFDATA" || ! -x "$LLVM_COV" ]]; then
  cat >&2 <<EOF
Error: llvm coverage tools not found in:
  $LLVM_BIN_DIR

Install the Rust LLVM tools component and retry:
  rustup component add llvm-tools-preview
EOF
  exit 1
fi

mkdir -p "$COVERAGE_DIR" "$CARGO_COVERAGE_TARGET_DIR"
mkdir -p "$PROFRAW_DIR"
rm -f "$COVERAGE_DIR"/"$PROFILE_PREFIX".profdata
rm -rf "$COVERAGE_DIR"/html

RUSTFLAGS_COVERAGE="-Cinstrument-coverage"
if [[ -n "${RUSTFLAGS:-}" ]]; then
  RUSTFLAGS_COVERAGE="${RUSTFLAGS} ${RUSTFLAGS_COVERAGE}"
fi

echo "[coverage] Running instrumented katex-rs-cli..."
RUN_LOG="$(mktemp)"
RUN_TAG="$(date +%s)-$$"
set +e
RUSTFLAGS="$RUSTFLAGS_COVERAGE" \
CARGO_INCREMENTAL=0 \
LLVM_PROFILE_FILE="$PROFRAW_DIR/$PROFILE_PREFIX-$RUN_TAG-%p-%m.profraw" \
CARGO_TARGET_DIR="$CARGO_COVERAGE_TARGET_DIR" \
cargo run --bin katex-rs-cli -- "$FORMULAS_FILE" --summary-only 2>&1 | tee "$RUN_LOG"
RUN_STATUS=${PIPESTATUS[0]}
set -e

if [[ "$RUN_STATUS" -ne 0 ]]; then
  if grep -q "Invalid cross-device link (os error 18)" "$RUN_LOG"; then
    FALLBACK_TARGET_DIR="/tmp/katex-wasm-coverage-target"
    echo "[coverage] Detected os error 18. Retrying with CARGO_TARGET_DIR=$FALLBACK_TARGET_DIR ..."
    CARGO_COVERAGE_TARGET_DIR="$FALLBACK_TARGET_DIR"
    mkdir -p "$CARGO_COVERAGE_TARGET_DIR"
    RUSTFLAGS="$RUSTFLAGS_COVERAGE" \
    CARGO_INCREMENTAL=0 \
    LLVM_PROFILE_FILE="$PROFRAW_DIR/$PROFILE_PREFIX-$RUN_TAG-%p-%m.profraw" \
    CARGO_TARGET_DIR="$CARGO_COVERAGE_TARGET_DIR" \
    cargo run --bin katex-rs-cli -- "$FORMULAS_FILE" --summary-only
  else
    rm -f "$RUN_LOG"
    echo "Error: instrumented katex-rs-cli run failed." >&2
    exit "$RUN_STATUS"
  fi
fi
rm -f "$RUN_LOG"

shopt -s nullglob
CURRENT_PROFRAW_FILES=("$PROFRAW_DIR"/"$PROFILE_PREFIX"-"$RUN_TAG"-*.profraw)
shopt -u nullglob
if [[ "${#CURRENT_PROFRAW_FILES[@]}" -eq 0 ]]; then
  echo "Error: no current-run .profraw files were generated in $PROFRAW_DIR" >&2
  exit 1
fi

MERGE_INPUT_FILES=("${CURRENT_PROFRAW_FILES[@]}")

for SPEC in "${MERGE_PROFRAW_SPECS[@]}"; do
  if [[ -d "$SPEC" ]]; then
    shopt -s nullglob
    EXTRA_FILES=("$SPEC"/*.profraw)
    shopt -u nullglob
    if [[ "${#EXTRA_FILES[@]}" -eq 0 ]]; then
      echo "Error: no .profraw files found under merge directory: $SPEC" >&2
      exit 1
    fi
    MERGE_INPUT_FILES+=("${EXTRA_FILES[@]}")
  elif [[ -f "$SPEC" && "$SPEC" == *.profraw ]]; then
    MERGE_INPUT_FILES+=("$SPEC")
  else
    echo "Error: --merge-profraw must be a .profraw file or a directory, got: $SPEC" >&2
    exit 1
  fi
done

echo "[coverage] Merging profraw files..."
"$LLVM_PROFDATA" merge -sparse "${MERGE_INPUT_FILES[@]}" -o "$COVERAGE_DIR/$PROFILE_PREFIX.profdata"

CLI_BIN="$CARGO_COVERAGE_TARGET_DIR/debug/katex-rs-cli"
if [[ ! -x "$CLI_BIN" ]]; then
  echo "Error: instrumented binary not found: $CLI_BIN" >&2
  exit 1
fi

echo "[coverage] Terminal summary:"
"$LLVM_COV" report "$CLI_BIN" \
  --instr-profile="$COVERAGE_DIR/$PROFILE_PREFIX.profdata" \
  --ignore-filename-regex="$COVERAGE_IGNORE_REGEX"

echo "[coverage] Generating HTML report..."
"$LLVM_COV" show "$CLI_BIN" \
  --instr-profile="$COVERAGE_DIR/$PROFILE_PREFIX.profdata" \
  --format=html \
  --output-dir="$COVERAGE_DIR/html" \
  --ignore-filename-regex="$COVERAGE_IGNORE_REGEX"

resolve_display_path() {
  local path="$1"
  if [[ "$path" = /* ]]; then
    echo "$path"
  else
    echo "$REPO_ROOT/$path"
  fi
}

echo "[coverage] Done."
echo "  profdata: $(resolve_display_path "$COVERAGE_DIR/$PROFILE_PREFIX.profdata")"
echo "  html:     $(resolve_display_path "$COVERAGE_DIR/html/index.html")"
echo "  profraw (current run): $(resolve_display_path "$PROFRAW_DIR")"
