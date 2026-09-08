#!/usr/bin/env bash
set -euo pipefail
wasmtime_bin="${WASMTIME_BIN:-wasmtime}"
wasm_bin="${WASI_CALCIT_BIN:-target/wasm32-wasip1/debug/wasi-calcit.wasm}"
# The fixture exercises bundled-core preprocessing, which used to exhaust
# wasm-ld's default shadow stack before user code could run (#763).
fixture_output="$("$wasmtime_bin" run --dir .::/workspace "$wasm_bin" --once /workspace/examples/calcit.cirru)"
printf '%s\n' "$fixture_output"
printf '%s\n' "$fixture_output" | grep -Eq '^took .*ms: 3$'
eval_output="$("$wasmtime_bin" run "$wasm_bin" --once -e '+ 1 2')"
printf '%s\n' "$eval_output"
printf '%s\n' "$eval_output" | grep -Eq '^took .*ms: 3$'
