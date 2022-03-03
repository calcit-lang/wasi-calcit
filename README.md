## WASI Calcit

> Calcit package on WASI

### Usage

```bash
wapm install calcit/wasi-calcit

wcr # trys to run `examples/compact.cirru`
```

_TODO_

### Develop

```bash
cargo build --target wasm32-wasi
wasmer run --mapdir examples/:examples/ target/wasm32-wasi/debug/wasi-calcit.wasm
```

or

```bash
cargo build --target wasm32-wasi
cp target/wasm32-wasi/release/wasi-calcit.wasm builds
# wasmer run --mapdir examples/:examples/ builds/wasi-calcit.wasm
wapm run wcr --dir examples/
```
