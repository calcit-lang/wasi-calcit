# Execute WASI host after migration / 迁移后实际执行 WASI 宿主

Tracks calcit-lang/calcit#753 and core #763. Restored preserved migration
2f32336 onto current main bea68d3 in a new worktree without modifying the
shared checkout. Calcit remains exactly =0.13.77; host version remains 0.0.0.

Core #763 identified wasm-ld's default 1 MiB shadow-stack exhaustion during
recursive bundled-core preprocessing. The upstream fix targets cr-wasm's
binary link, so it cannot supply this downstream host's link flags. build.rs
now reserves 4194304 bytes only for the wasi-calcit binary on wasm32-wasi*
targets. Native links are unchanged.

scripts/test-wasi.sh executes both the migrated Snapshot and eval under
Wasmtime, asserting each result is 3. CI installs official Wasmtime 18.0.3
(the original reproducer), verifies the downloaded Linux artifact SHA-256,
and runs that script with a bounded step. Existing native smoke, JS emission,
formatting, clippy and build gates remain. Action references use verified
release tags; exact Rust 1.97.1 is installed with rustup.

将上游已查明的 1 MiB 栈不足修复应用到宿主自身的最终链接阶段；不能假设
依赖的 cr-wasm 链接参数会传递给本二进制。新增真实执行回归，验证结果为 3，
保持版本和原有 gate；未修改共享工作目录。

Validation on Darwin arm64 / 本地验证：

- Rust 1.97.1 fmt, clippy --all-targets -- -D warnings, cargo test pass.
- wasm32-wasip1 debug build passes; Wasmtime 18.0.3 Snapshot and eval both
  execute and return 3, without the original out-of-bounds trap.
- Native Snapshot and eval both return 3; native JS emission exits 0.
- The four existing unresolved core async-FFI JS warnings remain exactly as
  recorded in the previous checkpoint; this host does not register those APIs.
- bash syntax and git diff checks pass. cargo test contains no unit tests;
  the executable native/WASI smokes provide the regression coverage here.

官方 Linux Wasmtime 工件 SHA-256：
1252dbd077fcdfc54da17a45860dc90c558f7f4d9301143bec9f59ce548301d8
