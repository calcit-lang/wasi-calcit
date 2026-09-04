## WASI Calcit

> Calcit `0.13.77` on WASI Preview 1, bundling [Calcit](https://github.com/calcit-lang/calcit) without the desktop watcher and platform injections.

- APIs <http://apis.calcit-lang.org/>
- Guide <http://guide.calcit-lang.org/>

### Usage

```bash
wapm install calcit/wasi-calcit

wcr --dir=. # runs `calcit.cirru` by default

wcr -e 'range 100' # eval mode
```

### Develop

```bash
cargo +1.97.1 build --target wasm32-wasip1
wasmer run --mapdir examples/:examples/ target/wasm32-wasip1/debug/wasi-calcit.wasm -- examples/calcit.cirru
```

> Calcit 0.13.77 currently builds for `wasm32-wasip1`, but execution is blocked
> by [calcit-lang/calcit#763](https://github.com/calcit-lang/calcit/issues/763),
> an upstream preprocessing memory trap also reproduced with the official
> `cr-wasm` binary.

or:

```bash
cargo +1.97.1 build --target wasm32-wasip1 --release
cp target/wasm32-wasip1/release/wasi-calcit.wasm builds
wapm run wcr -e 'range 100'
wapm run wcr --dir=examples examples/calcit.cirru
wapm run wcr --dir=./ examples/calcit.cirru --emit-js
```

### More...

Check out <https://github.com/calcit-lang/calcit-wasm-play> for browser version.
