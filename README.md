## WASI Calcit

> Calcit package on WASI. Based on Calcit-rs but without watcher and injections.

### Usage

```bash
wapm install calcit/wasi-calcit

wcr --dir=. # trys to run `compact.cirru`

wcr -e 'range 100' # eval mode
```

_TODO_

### Develop

```bash
cargo build --target wasm32-wasi
wasmer run --mapdir examples/:examples/ target/wasm32-wasi/debug/wasi-calcit.wasm -- examples/compact.cirru
```

or

```bash
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/wasi-calcit.wasm builds
wapm run wcr -e 'range 100'
wapm run wcr --dir=examples examples/compact.cirru
wapm run wcr --dir=./ examples/compact.cirru --emit-js
```
