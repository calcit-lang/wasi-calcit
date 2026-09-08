## WASI Calcit

> Calcit `0.13.77` on WASI Preview 1, bundling [Calcit](https://github.com/calcit-lang/calcit) without the desktop watcher and platform injections.

- APIs <http://apis.calcit-lang.org/>
- Guide <http://guide.calcit-lang.org/>

### Usage

```bash
wapm install calcit/wasi-calcit

wcr # runs `calcit.cirru` by default; grant directories in the WASI runtime

wcr -e 'range 100' # eval mode
```

### Develop

```bash
cargo +1.97.1 build --target wasm32-wasip1
wasmer run target/wasm32-wasip1/debug/wasi-calcit.wasm --volume .:/workspace -- /workspace/examples/calcit.cirru
```

The host reserves a 4 MiB WASI shadow stack when linking its binary. The
wasm-ld default 1 MiB stack is insufficient for bundled-core preprocessing
([calcit#763](https://github.com/calcit-lang/calcit/issues/763)). This must be
set by this host: the dependency's `cr-wasm` binary link arguments do not
apply to downstream binaries.

Use Wasmtime 18.0.3 (the original reproducer) to verify actual execution:

```bash
cargo +1.97.1 build --target wasm32-wasip1
bash scripts/test-wasi.sh
```

The script checks snapshot and eval results, both expected to be `3`.
`WASMTIME_BIN` can select a specific installation. WASI directory access is
granted by the runtime's `--dir .::/workspace` option, not a `wcr --dir`
application flag.

宿主在自己的链接阶段保留 4 MiB WASI 栈，并通过 Wasmtime 实际执行 Snapshot
和 eval 回归。目录授权属于 WASI runtime，依赖二进制的链接参数不会传递给宿主。

or:

```bash
cargo +1.97.1 build --target wasm32-wasip1 --release
cp target/wasm32-wasip1/release/wasi-calcit.wasm builds
wapm run wcr -e 'range 100'
wapm run wcr examples/calcit.cirru
wapm run wcr examples/calcit.cirru --emit-js
```

### More...

Check out <https://github.com/calcit-lang/calcit-wasm-play> for browser version.
