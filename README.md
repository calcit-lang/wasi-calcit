## WASI Calcit

> Calcit(`0.5.33`) package on WASI, bundled [Calcit-rs](https://github.com/calcit-lang/calcit) without watcher and injections.

- APIs <http://apis.calcit-lang.org/>
- Guide <http://guide.calcit-lang.org/>

### Usage

```bash
wapm install calcit/wasi-calcit

wcr --dir=. # trys to run `compact.cirru`

wcr -e 'range 100' # eval mode
```

### Develop

```bash
cargo build --target wasm32-wasi
wasmer run --mapdir examples/:examples/ target/wasm32-wasi/debug/wasi-calcit.wasm -- examples/compact.cirru
```

or:

```bash
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/wasi-calcit.wasm builds
wapm run wcr -e 'range 100'
wapm run wcr --dir=examples examples/compact.cirru
wapm run wcr --dir=./ examples/compact.cirru --emit-js
```

### More...

Check out <https://github.com/calcit-lang/calcit-wasm-play> for browser version.
